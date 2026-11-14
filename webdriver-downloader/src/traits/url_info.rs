use async_trait::async_trait;
use semver::{Version, VersionReq};

use crate::traits::version_req_url_info::VersionReqError;
use crate::traits::BoxError;

/// Error that can occur while getting the urls.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum UrlError {
    #[error("Failed to download Urls: {0}")]
    Download(#[source] BoxError),
    #[error(transparent)]
    VersionReq(#[from] VersionReqError),
    #[error("Failed to parse html: \"{0}\"")]
    Parse(String),
    #[error(transparent)]
    JsonParse(BoxError),
}

/// Fetches `url` and returns its body as text. A non-success status is an error, so that an error
/// page is never parsed as a version list.
pub(crate) async fn fetch_text(url: &str) -> Result<String, UrlError> {
    let download = |e: reqwest::Error| UrlError::Download(e.into());

    reqwest::get(url)
        .await
        .map_err(download)?
        .error_for_status()
        .map_err(download)?
        .text()
        .await
        .map_err(download)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct WebdriverVersionUrl {
    pub version_req: VersionReq,
    pub webdriver_version: Version,
    pub url: String,
}

/// Provides information for determining which url to download.
#[async_trait]
pub(crate) trait WebdriverUrlInfo {
    /// Lists viable [`WebdriverVersionUrl`]s, up to `limit`.
    async fn version_urls(&self, limit: usize) -> Result<Vec<WebdriverVersionUrl>, UrlError>;
}
