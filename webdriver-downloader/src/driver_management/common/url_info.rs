use async_trait::async_trait;
use semver::Version;

use crate::common::version_req_url_info::VersionReqError;

#[derive(thiserror::Error, Debug)]
pub enum UrlError {
    #[error("Failed to download Urls: {0}")]
    Download(#[from] reqwest::Error),
    #[error(transparent)]
    BinaryVersion(#[from] VersionReqError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VersionUrl {
    pub version: Version,
    pub url: String,
}

/// Provides information for determining which url to download.
#[async_trait]
pub trait WebdriverUrlInfo {
    /// Lists viable VersionUrls, up to `limit`.
    async fn version_urls(&self, limit: usize) -> Result<Vec<VersionUrl>, UrlError>;
}
