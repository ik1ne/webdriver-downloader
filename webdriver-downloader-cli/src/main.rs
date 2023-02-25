use anyhow::Result;

mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    cli::run()
        .await
        .map(|_| println!("Webdriver installed successfully."))
}
