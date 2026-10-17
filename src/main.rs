use clap::Parser;
use std::path::PathBuf;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser, Debug)]
#[command(name = "pipeline-fs", author, version, about = "Multi-stage cascading FUSE filesystem with tier demotion")]
struct Cli {
    #[arg(short, long, value_name = "DIR", help = "Mount point directory")]
    mount_point: PathBuf,

    #[arg(short, long, value_name = "STAGES", num_args = 1.., help = "Ordered stage directories (null, 0, 1, ... N)")]
    stages: Vec<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let cli = Cli::parse();

    info!("Starting pipeline-fs...");
    info!("Mount Point: {:?}", cli.mount_point);
    info!("Stages configured: {:?}", cli.stages);

    if cli.stages.is_empty() {
        anyhow::bail!("At least one stage directory must be specified.");
    }

    println!("pipeline-fs initialized successfully. Ready for mounting at {:?}", cli.mount_point);
    Ok(())
}
