use anyhow::Result;
use std::path::Path;

use webdriver_downloader::cli;

#[tokio::main]
async fn main() -> Result<()> {
    cli::run().await
}
