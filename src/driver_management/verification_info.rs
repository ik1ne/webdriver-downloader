use std::ffi::OsStr;
use std::path::Path;

use anyhow::Result;
use async_trait::async_trait;
use fantoccini::wd::Capabilities;
use fantoccini::Locator;

/// Provides information for verifying an installed driver.
#[async_trait]
pub trait WebdriverVerificationInfo {
    /// Capabilities to use for verification.
    /// Some driver options such as browser path can be provided by capabilities.
    fn driver_capabilities(&self) -> Option<Capabilities>;

    /// Verifies driver using [test_client](WebdriverVerificationInfo::test_client).
    async fn verify_driver<P: AsRef<Path> + Sync>(&self, driver_path: &P) -> Result<()> {
        let _driver_command = tokio::process::Command::new(OsStr::new(driver_path.as_ref()))
            .arg("--port=4444")
            .kill_on_drop(true)
            .spawn()?;

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

        let _ = client.close().await;
        test_result
    }

    async fn test_client(client: &fantoccini::Client) -> Result<()> {
        client.goto("https://www.example.com").await?;
        client.find(Locator::Css("html")).await?;

        Ok(())
    }
}
