use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use warp::Filter;

#[derive(Parser, Debug)]
#[command(name = "pipeline-fs", author, version, about = "Multi-stage cascading FUSE filesystem with tier demotion")]
struct Cli {
    #[arg(short, long, value_name = "DIR", help = "Mount point directory")]
    mount_point: PathBuf,

    #[arg(short, long, value_name = "STAGES", num_args = 1.., help = "Ordered stage directories (source, 0, 1, ... N)")]
    stages: Vec<PathBuf>,

    #[arg(short, long, value_name = "PORT", help = "HTTP API port")]
    port: Option<u16>,
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

async fn health_check() -> Result<impl warp::Reply, warp::Rejection> {
    let response = HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        capabilities: Capabilities {
            health_endpoint: true,
            config_endpoint: false,
            stages_endpoint: false,
            files_endpoint: false,
            migrate_endpoint: false,
            events_endpoint: false,
            policies_endpoint: false,
        },
    };
    
    Ok(warp::reply::json(&response))
}

#[cfg(test)]
mod tests {
    use super::health_check;
    use warp::Reply;

    #[tokio::test]
    async fn health_response_reports_registered_capabilities() {
        let response = health_check().await.unwrap().into_response();
        assert_eq!(response.status(), warp::http::StatusCode::OK);

        let body = warp::hyper::body::to_bytes(response.into_body())
            .await
            .unwrap();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["status"], "healthy");
        assert_eq!(body["capabilities"]["healthEndpoint"], true);
        assert_eq!(body["capabilities"]["configEndpoint"], false);
        assert_eq!(body["capabilities"]["filesEndpoint"], false);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    println!("pipeline-fs initialized successfully.");
    println!("Mount Point: {:?}", cli.mount_point);
    println!("Stages configured: {:?}", cli.stages);

    if cli.stages.is_empty() {
        eprintln!("Error: At least one stage directory must be specified.");
        std::process::exit(1);
    }

    // Start the HTTP server if a port is specified
    if let Some(port) = cli.port {
        println!("Starting HTTP server on port {}", port);
        
        // Create the health check endpoint
        let health = warp::path("health").and_then(health_check);
        
        // Combine all routes
        let routes = health;
        
        // Start the server
        warp::serve(routes)
            .run(([0, 0, 0, 0], port))
            .await;
    }

    Ok(())
}
