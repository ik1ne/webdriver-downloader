use async_trait::async_trait;

use crate::traits::binary_major_version_hint_url_info::BinaryVersionError;

#[derive(thiserror::Error, Debug)]
pub enum UrlError {
    #[error("Failed to download Urls: {0}")]
    Download(#[from] reqwest::Error),
    #[error(transparent)]
    BinaryVersion(#[from] BinaryVersionError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Provides information for determining which url to download.
#[async_trait]
pub trait WebdriverUrlInfo {
    /// Lists viable driver urls, up to `limit`.
    async fn driver_urls(&self, limit: usize) -> Result<Vec<String>, UrlError>;
}
