use anyhow::Result;

mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    cli::run().await.map(|s| println!("{}", s))
}
