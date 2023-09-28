use async_trait::async_trait;
use semver::{Version, VersionReq};

use crate::traits::version_req_url_info::VersionReqError;

/// Error that can occur while getting the urls.
#[derive(thiserror::Error, Debug)]
pub enum UrlError {
    #[error("Failed to download Urls: {0}")]
    Download(#[from] reqwest::Error),
    #[error(transparent)]
    VersionReq(#[from] VersionReqError),
    #[error("Failed to parse html: \"{0}\"")]
    Parse(String),
    #[error(transparent)]
    Version(#[from] lenient_semver::parser::OwnedError),
    #[error(transparent)]
    JsonParse(#[from] serde_json::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WebdriverVersionUrl {
    pub version_req: VersionReq,
    pub webdriver_version: Version,
    pub url: String,
}

/// Provides information for determining which url to download.
#[async_trait]
pub trait WebdriverUrlInfo {
    /// Lists viable [`WebdriverVersionUrl`]s, up to `limit`.
    async fn version_urls(&self, limit: usize) -> Result<Vec<WebdriverVersionUrl>, UrlError>;
}
