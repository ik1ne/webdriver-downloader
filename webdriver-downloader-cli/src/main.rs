use anyhow::Result;
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;

mod cli;

fn main() -> Result<()> {
    tracing_subscriber::fmt::fmt()
        .with_span_events(FmtSpan::NEW)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Starting webdriver-downloader-cli");

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime");

    runtime.block_on(async { cli::run().await.map(|s| println!("{}", s)) })
}
