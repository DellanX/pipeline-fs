use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::net::TcpListener;
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
}

async fn health_check() -> Result<impl warp::Reply, warp::Rejection> {
    let response = HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    };
    
    Ok(warp::reply::json(&response))
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
