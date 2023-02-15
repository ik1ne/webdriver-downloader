use anyhow::Result;
use async_trait::async_trait;

/// Provides information for determining which url to download.
#[async_trait]
pub trait WebdriverUrlInfo {
    /// Lists viable driver urls, up to `limit`.
    async fn driver_urls(&self, limit: usize) -> Result<Vec<String>>;
}
