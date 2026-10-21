use clap::Parser;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use warp::http::StatusCode;
use warp::Filter;
use warp::Reply;

type HttpResponse = warp::reply::Response;

#[derive(Parser, Debug)]
#[command(
    name = "pipeline-fs",
    author,
    version,
    about = "Multi-stage cascading FUSE filesystem with tier demotion"
)]
struct Cli {
    #[arg(short, long, value_name = "STAGES", num_args = 1.., help = "Ordered stage directories (source, 0, 1, ... N)")]
    stages: Vec<PathBuf>,

    #[arg(short, long, value_name = "PORT", help = "HTTP API port")]
    port: Option<u16>,

    #[arg(
        short,
        long,
        value_name = "CONFIG",
        help = "Configuration file path (TOML format)"
    )]
    config: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    stages: Vec<PathBuf>,
    port: Option<u16>,
    mount_point: Option<PathBuf>,
}

#[derive(Serialize, Deserialize)]
struct HealthResponse {
    status: String,
    version: String,
    timestamp: u64,
    capabilities: Capabilities,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Capabilities {
    health_endpoint: bool,
    config_endpoint: bool,
    stages_endpoint: bool,
    files_endpoint: bool,
    migrate_endpoint: bool,
    events_endpoint: bool,
    policies_endpoint: bool,
}

async fn health_check(config_endpoint_enabled: bool) -> Result<impl warp::Reply, warp::Rejection> {
    let response = HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        capabilities: Capabilities {
            health_endpoint: true,
            config_endpoint: config_endpoint_enabled,
            stages_endpoint: false,
            files_endpoint: false,
            migrate_endpoint: false,
            events_endpoint: false,
            policies_endpoint: false,
        },
    };

    Ok(warp::reply::json(&response))
}

async fn get_config(config_path: PathBuf, enabled: bool) -> Result<HttpResponse, Infallible> {
    if !enabled {
        return Ok(warp::reply::with_status("", StatusCode::NOT_FOUND).into_response());
    }

    match load_config(&config_path) {
        Ok(config) => Ok(warp::reply::json(&config).into_response()),
        Err(error) => Ok(config_error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read configuration: {error}"),
        )),
    }
}

async fn put_config(
    config_path: PathBuf,
    enabled: bool,
    config: Config,
) -> Result<HttpResponse, Infallible> {
    if !enabled {
        return Ok(warp::reply::with_status("", StatusCode::NOT_FOUND).into_response());
    }

    if config.stages.is_empty() {
        return Ok(config_error_response(
            StatusCode::BAD_REQUEST,
            "At least one stage directory must be specified.".to_string(),
        ));
    }

    let contents = match toml::to_string_pretty(&config) {
        Ok(contents) => contents,
        Err(error) => {
            return Ok(config_error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to serialize configuration: {error}"),
            ));
        }
    };

    match write_config(&config_path, contents.as_bytes()) {
        Ok(()) => Ok(warp::reply::json(&config).into_response()),
        Err(error) => Ok(config_error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to write configuration: {error}"),
        )),
    }
}

#[derive(Serialize)]
struct ConfigError {
    error: String,
}

fn config_error_response(status: StatusCode, message: String) -> HttpResponse {
    warp::reply::with_status(warp::reply::json(&ConfigError { error: message }), status)
        .into_response()
}

fn write_config(config_path: &Path, contents: &[u8]) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(config_path)?;
    file.write_all(contents)?;
    file.sync_all()
}

fn config_route(
    config_path: PathBuf,
    enabled: bool,
) -> impl Filter<Extract = (HttpResponse,), Error = warp::Rejection> + Clone {
    let get_config = warp::get()
        .and(warp::any().map({
            let config_path = config_path.clone();
            move || (config_path.clone(), enabled)
        }))
        .and_then(|(path, enabled)| get_config(path, enabled));
    let put_config = warp::put()
        .and(warp::body::content_length_limit(64 * 1024))
        .and(warp::body::json())
        .and(warp::any().map(move || (config_path.clone(), enabled)))
        .and_then(|config, (path, enabled)| put_config(path, enabled, config));

    warp::path!("api" / "v1" / "config")
        .and(warp::path::end())
        .and(get_config.or(put_config).unify())
}

fn config_file_is_writable(config_path: &Path) -> std::io::Result<()> {
    OpenOptions::new().write(true).open(config_path).map(drop)
}

fn load_config(config_path: &PathBuf) -> anyhow::Result<Config> {
    let config_content = std::fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Load configuration from file if provided
    let mut config = Config {
        stages: Vec::new(),
        port: None,
        mount_point: None,
    };
    let mut config_endpoint_enabled = false;
    
    // Default to config.toml if no config is specified
    let config_path = cli.config.clone().unwrap_or_else(|| PathBuf::from("config.toml"));

    let loaded_config_result = load_config(&config_path);
    
    match loaded_config_result {
        Ok(loaded_config) => {
            config.stages = loaded_config.stages;
            config.port = loaded_config.port.or(cli.port);
            match config_file_is_writable(&config_path) {
                Ok(()) => config_endpoint_enabled = true,
                Err(error) => {
                    eprintln!(
                        "Configuration endpoint disabled: cannot write config file '{}': {}",
                        config_path.display(),
                        error
                    );
                }
            }
            // Note: mount_point from config is not used in current implementation
        }
        Err(_) => {
            // If config file doesn't exist, use CLI arguments as fallback
            config.stages = cli.stages;
            config.port = cli.port;
            if cli.config.is_none() {
                eprintln!("No config file found at '{}'. Using CLI arguments for configuration.", config_path.display());
            }
        }
    };

    println!("pipeline-fs initialized successfully.");
    println!("Stages configured: {:?}", config.stages);

    if config.stages.is_empty() {
        eprintln!("Error: At least one stage directory must be specified.");
        std::process::exit(1);
    }

    // Start the HTTP server if a port is specified
    if let Some(port) = config.port {
        println!("Starting HTTP server on port {}", port);

        // Add CORS support for all routes
        let cors = warp::cors()
            .allow_any_origin()
            .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allow_headers(vec!["Content-Type", "Authorization"]);

        // Route all non-API paths to the UI
        let ui = warp::fs::dir("ui/dist/");

        // Combine all routes - API routes must be matched before UI routes
        let api_routes = warp::get()
            .and(warp::path("api").and(warp::path("v1")).and(warp::path("health")))
            .and(warp::path::end())
            .and_then(move || health_check(config_endpoint_enabled))
            .or(config_route(
                config_path.clone(),
                config_endpoint_enabled,
            ))
            .with(cors);

        let all_routes = warp::get()
            .and(warp::path::end())
            .and(ui)
            .or(api_routes);

        // Start the server
        warp::serve(all_routes).run(([0, 0, 0, 0], port)).await;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config_path() -> PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "pipeline-fs-config-{}-{unique}.toml",
            std::process::id()
        ))
    }

    #[tokio::test]
    async fn config_endpoint_get_returns_current_configuration() {
        let config_path = test_config_path();
        std::fs::write(&config_path, "stages = [\"/tmp/stage0\"]\nport = 8080\n").unwrap();

        let response = warp::test::request()
            .method("GET")
            .path("/api/v1/config")
            .reply(&config_route(config_path.clone(), true))
            .await;
        let config: serde_json::Value = serde_json::from_slice(response.body()).unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(config["stages"][0], "/tmp/stage0");
        assert_eq!(config["port"], 8080);
        std::fs::remove_file(config_path).unwrap();
    }

    #[tokio::test]
    async fn config_endpoint_put_persists_configuration() {
        let config_path = test_config_path();
        std::fs::write(&config_path, "stages = [\"/tmp/stage0\"]\n").unwrap();
        let new_config = Config {
            stages: vec![PathBuf::from("/tmp/new-stage")],
            port: Some(9090),
            mount_point: None,
        };

        let response = warp::test::request()
            .method("PUT")
            .path("/api/v1/config")
            .json(&new_config)
            .reply(&config_route(config_path.clone(), true))
            .await;
        let persisted = load_config(&config_path).unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(persisted.stages, new_config.stages);
        assert_eq!(persisted.port, new_config.port);
        std::fs::remove_file(config_path).unwrap();
    }

    #[tokio::test]
    async fn config_endpoint_put_rejects_empty_stages() {
        let config_path = test_config_path();
        std::fs::write(&config_path, "stages = [\"/tmp/stage0\"]\n").unwrap();
        let invalid_config = Config {
            stages: Vec::new(),
            port: None,
            mount_point: None,
        };

        let response = warp::test::request()
            .method("PUT")
            .path("/api/v1/config")
            .json(&invalid_config)
            .reply(&config_route(config_path.clone(), true))
            .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        std::fs::remove_file(config_path).unwrap();
    }

    #[tokio::test]
    async fn config_endpoint_is_unavailable_when_disabled() {
        let config_path = test_config_path();
        let response = warp::test::request()
            .method("GET")
            .path("/api/v1/config")
            .reply(&config_route(config_path, false))
            .await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn config_endpoint_allows_cors_put_preflight() {
        let response = warp::test::request()
            .method("OPTIONS")
            .path("/api/v1/config")
            .header("origin", "http://localhost:5173")
            .header("access-control-request-method", "PUT")
            .reply(
                &config_route(test_config_path(), true).with(
                    warp::cors()
                        .allow_any_origin()
                        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allow_headers(vec!["Content-Type", "Authorization"]),
                ),
            )
            .await;

        assert_eq!(response.status(), StatusCode::OK);
        assert!(response
            .headers()
            .get("access-control-allow-methods")
            .unwrap()
            .to_str()
            .unwrap()
            .contains("PUT"));
    }

    #[tokio::test]
    async fn health_reports_config_endpoint_availability() {
        let response = health_check(true).await.unwrap().into_response();
        let body = warp::hyper::body::to_bytes(response.into_body())
            .await
            .unwrap();
        let health: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(health["capabilities"]["configEndpoint"], true);
    }
}
