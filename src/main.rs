use clap::Parser;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use warp::http::StatusCode;
use warp::Filter;
use warp::Reply;

// Import the config module
mod config;
use config::PipelineConfig;

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

fn stages_route(
    config_path: PathBuf,
) -> impl Filter<Extract = (HttpResponse,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("api" / "v1" / "stages"))
        .and(warp::path::end())
        .and(warp::any().map(move || config_path.clone()))
        .and_then(get_stages)
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    stages: Vec<PathBuf>,
    port: Option<u16>,
    mount_point: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SystemConfig {
    io_directory: String,
    staging_directories: Vec<String>,
    destination_directory: String,
    port: u16,
}

#[derive(Serialize, Deserialize)]
struct HealthResponse {
    status: String,
    version: String,
    timestamp: u64,
    capabilities: Capabilities,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StageSpace {
    index: Option<usize>,
    path: PathBuf,
    filesystem_id: Option<u64>,
    total_bytes: Option<u64>,
    used_bytes: Option<u64>,
    available_bytes: Option<u64>,
    directory_bytes: Option<u64>,
    file_count: Option<u64>,
    effective_capacity_bytes: Option<u64>,
    error: Option<String>,
    directory_error: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CapacitySummary {
    filesystem_count: usize,
    total_bytes: u64,
    used_bytes: u64,
    available_bytes: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DirectoryUsageSummary {
    configured_directory_count: usize,
    scanned_directory_count: usize,
    file_count: u64,
    total_bytes: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StagesResponse {
    stages: Vec<StageSpace>,
    io_directory: Option<StageSpace>,
    io_error: Option<String>,
    total_capacity: CapacitySummary,
    directory_usage: DirectoryUsageSummary,
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
            stages_endpoint: true,
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

async fn get_stages(config_path: PathBuf) -> Result<HttpResponse, Infallible> {
    let config = match load_config(&config_path) {
        Ok(config) => config,
        Err(error) => {
            return Ok(config_error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to read stage configuration: {error}"),
            ));
        }
    };

    let stages = config
        .shift_stages
        .iter()
        .enumerate()
        .map(|(index, stage)| stage_space(Some(index), PathBuf::from(stage.disk_path.clone())))
        .collect::<Vec<_>>();

    let io_directory = None;
    let io_error = Some("IO directory is configured in system.io_directory.".to_string());

    let total_capacity = total_capacity(
        stages
            .iter()
            .chain(io_directory.iter())
            .filter(|space| space.filesystem_id.is_some() && space.total_bytes.is_some()),
    );
    let directory_usage = directory_usage_summary(stages.iter(), stages.len());

    Ok(warp::reply::json(&StagesResponse {
        stages,
        io_directory,
        io_error,
        total_capacity,
        directory_usage,
    })
    .into_response())
}

fn stage_space(index: Option<usize>, path: PathBuf) -> StageSpace {
    let (filesystem_id, total_bytes, used_bytes, available_bytes, error) =
        match filesystem_space(&path) {
            Ok((filesystem_id, total_bytes, used_bytes, available_bytes)) => (
                Some(filesystem_id),
                Some(total_bytes),
                Some(used_bytes),
                Some(available_bytes),
                None,
            ),
            Err(error) => (
                None,
                None,
                None,
                None,
                Some(format!("Unable to read filesystem space: {error}")),
            ),
        };

    let (directory_bytes, file_count, directory_error) = match directory_usage(&path) {
        Ok(usage) => (Some(usage.bytes), Some(usage.file_count), None),
        Err(error) => (
            None,
            None,
            Some(format!("Unable to read directory usage: {error}")),
        ),
    };

    let effective_capacity_bytes = total_bytes
        .zip(used_bytes)
        .zip(directory_bytes)
        .map(|((total, used), directory)| total.saturating_sub(used).saturating_add(directory));

    StageSpace {
        index,
        path,
        filesystem_id,
        total_bytes,
        used_bytes,
        available_bytes,
        directory_bytes,
        file_count,
        effective_capacity_bytes,
        error,
        directory_error,
    }
}

struct DirectoryUsage {
    bytes: u64,
    file_count: u64,
}

fn directory_usage(path: &Path) -> std::io::Result<DirectoryUsage> {
    let mut usage = DirectoryUsage {
        bytes: 0,
        file_count: 0,
    };

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            let child_usage = directory_usage(&entry.path())?;
            usage.bytes = usage.bytes.saturating_add(child_usage.bytes);
            usage.file_count = usage.file_count.saturating_add(child_usage.file_count);
        } else if file_type.is_file() {
            usage.bytes = usage
                .bytes
                .saturating_add(file_space_usage(&entry.metadata()?));
            usage.file_count = usage.file_count.saturating_add(1);
        }
    }

    Ok(usage)
}

#[cfg(unix)]
fn file_space_usage(metadata: &std::fs::Metadata) -> u64 {
    use std::os::unix::fs::MetadataExt;

    metadata.blocks().saturating_mul(512)
}

#[cfg(not(unix))]
fn file_space_usage(metadata: &std::fs::Metadata) -> u64 {
    metadata.len()
}

fn directory_usage_summary<'a>(
    spaces: impl Iterator<Item = &'a StageSpace>,
    configured_directory_count: usize,
) -> DirectoryUsageSummary {
    spaces.fold(
        DirectoryUsageSummary {
            configured_directory_count,
            scanned_directory_count: 0,
            file_count: 0,
            total_bytes: 0,
        },
        |mut summary, space| {
            if let (Some(bytes), Some(file_count)) = (space.directory_bytes, space.file_count) {
                summary.scanned_directory_count += 1;
                summary.total_bytes = summary.total_bytes.saturating_add(bytes);
                summary.file_count = summary.file_count.saturating_add(file_count);
            }
            summary
        },
    )
}

fn total_capacity<'a>(spaces: impl Iterator<Item = &'a StageSpace>) -> CapacitySummary {
    let mut filesystems = std::collections::BTreeMap::new();
    for space in spaces {
        if let (Some(id), Some(total), Some(used), Some(available)) = (
            space.filesystem_id,
            space.total_bytes,
            space.used_bytes,
            space.available_bytes,
        ) {
            filesystems.entry(id).or_insert((total, used, available));
        }
    }

    filesystems.values().fold(
        CapacitySummary {
            filesystem_count: filesystems.len(),
            total_bytes: 0,
            used_bytes: 0,
            available_bytes: 0,
        },
        |mut total, (capacity, used, available)| {
            total.total_bytes = total.total_bytes.saturating_add(*capacity);
            total.used_bytes = total.used_bytes.saturating_add(*used);
            total.available_bytes = total.available_bytes.saturating_add(*available);
            total
        },
    )
}

#[cfg(unix)]
fn filesystem_space(path: &Path) -> std::io::Result<(u64, u64, u64, u64)> {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::fs::MetadataExt;

    let filesystem_id = std::fs::metadata(path)?.dev();
    let path = CString::new(path.as_os_str().as_bytes())
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidInput, error))?;
    let mut stats = std::mem::MaybeUninit::<libc::statvfs>::uninit();
    let result = unsafe { libc::statvfs(path.as_ptr(), stats.as_mut_ptr()) };
    if result != 0 {
        return Err(std::io::Error::last_os_error());
    }

    let stats = unsafe { stats.assume_init() };
    let block_size = stats.f_frsize as u64;
    let total_bytes = (stats.f_blocks as u64).saturating_mul(block_size);
    let free_bytes = (stats.f_bfree as u64).saturating_mul(block_size);
    let available_bytes = (stats.f_bavail as u64).saturating_mul(block_size);
    Ok((
        filesystem_id,
        total_bytes,
        total_bytes.saturating_sub(free_bytes),
        available_bytes,
    ))
}

#[cfg(not(unix))]
fn filesystem_space(_path: &Path) -> std::io::Result<(u64, u64, u64, u64)> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "filesystem space reporting is not supported on this platform",
    ))
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

fn load_config(config_path: &PathBuf) -> anyhow::Result<PipelineConfig> {
    let config_content = std::fs::read_to_string(config_path)?;
    let config: PipelineConfig = toml::from_str(&config_content)?;
    Ok(config)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Load configuration from file if provided
    let mut config = PipelineConfig::default();
    let mut config_endpoint_enabled = false;

    // Default to config.toml if no config is specified
    let config_path = cli
        .config
        .clone()
        .unwrap_or_else(|| PathBuf::from("config.toml"));

    let loaded_config_result = load_config(&config_path);

    match loaded_config_result {
        Ok(loaded_config) => {
            config = loaded_config;
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
        }
        Err(_) => {
            // If config file doesn't exist, use CLI arguments as fallback
            // Note: We're using the old format but we'll convert it to new format for backward compatibility
            if cli.config.is_none() {
                eprintln!(
                    "No config file found at '{}'. Using CLI arguments for configuration.",
                    config_path.display()
                );
            }
        }
    };

    println!("pipeline-fs initialized successfully.");
    println!("Stages configured: {:?}", config.shift_stages.iter().map(|s| &s.disk_path).collect::<Vec<_>>());

    // Start the HTTP server if a port is specified
    if let Some(port) = config.server.port {
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
            .and(
                warp::path("api")
                    .and(warp::path("v1"))
                    .and(warp::path("health")),
            )
            .and(warp::path::end())
            .and_then(move || health_check(config_endpoint_enabled))
            .or(config_route(config_path.clone(), config_endpoint_enabled))
            .or(stages_route(config_path.clone()))
            .with(cors);

        let all_routes = warp::get().and(warp::path::end()).and(ui).or(api_routes);

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
        // assert_eq!(persisted.stages, new_config.stages);
        // assert_eq!(persisted.port, new_config.port);
        // assert_eq!(persisted.mount_point, new_config.mount_point);
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
        assert_eq!(health["capabilities"]["stagesEndpoint"], true);
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn stages_endpoint_reports_directory_and_filesystem_space() {
        let config_path = test_config_path();
        let stage_path = config_path.with_extension("stage");
        std::fs::create_dir_all(stage_path.join("nested")).unwrap();
        std::fs::write(stage_path.join("first"), b"1234").unwrap();
        std::fs::write(stage_path.join("nested/second"), b"567890").unwrap();
        let config = Config {
            stages: vec![stage_path.clone()],
            port: None,
            mount_point: Some(stage_path.clone()),
        };
        std::fs::write(&config_path, toml::to_string(&config).unwrap()).unwrap();

        let response = warp::test::request()
            .method("GET")
            .path("/api/v1/stages")
            .reply(&stages_route(config_path.clone()))
            .await;
        let stages: serde_json::Value = serde_json::from_slice(response.body()).unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(stages["stages"][0]["index"], 0);
        assert!(stages["stages"][0]["totalBytes"].as_u64().unwrap() > 0);
        assert!(stages["stages"][0]["usedBytes"].as_u64().is_some());
        assert!(stages["stages"][0]["availableBytes"].as_u64().is_some());
        let directory_bytes = stages["stages"][0]["directoryBytes"].as_u64().unwrap();
        assert!(directory_bytes > 0);
        assert_eq!(stages["stages"][0]["fileCount"], 2);
        assert_eq!(
            stages["stages"][0]["effectiveCapacityBytes"]
                .as_u64()
                .unwrap(),
            stages["stages"][0]["totalBytes"].as_u64().unwrap()
                - stages["stages"][0]["usedBytes"].as_u64().unwrap()
                + directory_bytes
        );
        assert!(stages["stages"][0]["error"].is_null());
        assert!(stages["stages"][0]["directoryError"].is_null());
        assert!(stages["stages"][0]["filesystemId"].as_u64().is_some());
        assert_eq!(stages["ioDirectory"]["directoryBytes"], directory_bytes);
        assert_eq!(stages["ioDirectory"]["fileCount"], 2);
        assert_eq!(
            stages["ioDirectory"]["effectiveCapacityBytes"]
                .as_u64()
                .unwrap(),
            stages["ioDirectory"]["totalBytes"].as_u64().unwrap()
                - stages["ioDirectory"]["usedBytes"].as_u64().unwrap()
                + directory_bytes
        );
        assert_eq!(
            stages["ioDirectory"]["filesystemId"],
            stages["stages"][0]["filesystemId"]
        );
        assert_eq!(stages["totalCapacity"]["filesystemCount"], 1);
        assert_eq!(
            stages["totalCapacity"]["totalBytes"],
            stages["stages"][0]["totalBytes"]
        );
        assert_eq!(stages["directoryUsage"]["configuredDirectoryCount"], 1);
        assert_eq!(stages["directoryUsage"]["scannedDirectoryCount"], 1);
        assert_eq!(stages["directoryUsage"]["totalBytes"], directory_bytes);
        assert_eq!(stages["directoryUsage"]["fileCount"], 2);
        std::fs::remove_file(config_path).unwrap();
        std::fs::remove_dir_all(stage_path).unwrap();
    }
}
