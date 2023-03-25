use std::ffi::OsStr;
use std::path::Path;
use std::process::Child;

use async_trait::async_trait;
use fantoccini::wd::Capabilities;
use fantoccini::Locator;

struct ChildGuard(Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        if let Err(e) = self.0.kill() {
            println!("Failed to kill child process: {}", e);
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum VerificationError {
    #[error("Failed to start driver: {0}")]
    Start(#[from] std::io::Error),
    #[error("Failed to connect to driver: {0}")]
    Connect(#[from] fantoccini::error::NewSessionError),
    #[error("Driver test failed to pass: {0}")]
    Navigate(#[from] fantoccini::error::CmdError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Provides information for verifying a webdriver.
#[async_trait]
pub trait WebdriverVerificationInfo {
    /// Capabilities to use for verification.
    /// Some driver options such as browser path can be provided by capabilities.
    fn driver_capabilities(&self) -> Option<Capabilities>;

    /// Verifies driver using [test_client](WebdriverVerificationInfo::test_client).
    async fn verify_driver<P: AsRef<Path> + Sync>(
        &self,
        driver_path: &P,
    ) -> Result<(), VerificationError> {
        let _child = ChildGuard(
            std::process::Command::new(OsStr::new(driver_path.as_ref()))
                .arg("--port=4444")
                .spawn()?,
        );

        let client;
        if let Some(capabilities) = self.driver_capabilities() {
            client = fantoccini::ClientBuilder::native()
                .capabilities(capabilities)
                .connect("http://localhost:4444")
                .await?;
        } else {
            client = fantoccini::ClientBuilder::native()
                .connect("http://localhost:4444")
                .await?;
        }

        let test_result = Self::test_client(&client).await;

        if let Err(e) = client.close().await {
            println!("Failed to close client: {}", e);
        }

        test_result
    }

    async fn test_client(client: &fantoccini::Client) -> Result<(), VerificationError> {
        client.goto("https://www.example.com").await?;
        client.find(Locator::Css("html")).await?;

        Ok(())
    }
}
