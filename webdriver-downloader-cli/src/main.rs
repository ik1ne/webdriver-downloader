use anyhow::Result;

mod cli;

fn main() -> Result<()> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime");

    runtime.block_on(async { cli::run().await.map(|s| println!("{}", s)) })
}
