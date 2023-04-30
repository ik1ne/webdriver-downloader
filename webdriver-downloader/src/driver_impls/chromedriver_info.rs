use std::path::{Path, PathBuf};

use async_trait::async_trait;
use fantoccini::wd::Capabilities;
use regex::Regex;
use semver::{Version, VersionReq};
use serde_json::{json, Map};

use crate::os_specific;
use crate::os_specific::DefaultPathError;
use crate::traits::installation_info::WebdriverInstallationInfo;
use crate::traits::url_info::{UrlError, WebdriverVersionUrl};
use crate::traits::verification_info::WebdriverVerificationInfo;
use crate::traits::version_req_url_info::{VersionReqError, VersionReqUrlInfo};

/// Information required to implement [WebdriverDownloadInfo](crate::prelude::WebdriverDownloadInfo) for Chromedriver.
pub struct ChromedriverInfo {
    driver_install_path: PathBuf,
    browser_path: PathBuf,
}

impl ChromedriverInfo {
    pub fn new(driver_install_path: PathBuf, browser_path: PathBuf) -> Self {
        ChromedriverInfo {
            driver_install_path,
            browser_path,
        }
    }

    /// Initialize ChromedriverInfo with default paths.
    ///
    /// # Errors
    ///
    /// Returns [`DefaultPathError`] if the default paths cannot be determined.
    pub fn new_default() -> Result<Self, DefaultPathError> {
        let driver_install_path = os_specific::chromedriver::default_driver_path()?;
        let browser_path = os_specific::chromedriver::default_browser_path()?;
        Ok(ChromedriverInfo::new(driver_install_path, browser_path))
    }
}

#[async_trait]
impl VersionReqUrlInfo for ChromedriverInfo {
    fn binary_version(&self) -> Result<Version, VersionReqError> {
        os_specific::chromedriver::binary_version(&self.browser_path)
    }

    async fn driver_version_urls(&self) -> Result<Vec<WebdriverVersionUrl>, UrlError> {
        let download_xml = "https://chromedriver.storage.googleapis.com";

        let xml = reqwest::get(download_xml).await?.text().await?;

        let re =
            Regex::new(os_specific::chromedriver::ZIPFILE_NAME_RE).expect("Failed to parse regex.");

        let mut versions: Vec<WebdriverVersionUrl> = vec![];
        for captures in re.captures_iter(&xml) {
            let or_else =
                || VersionReqError::RegexError(captures.get(0).unwrap().as_str().to_string());

            let version_str = captures.get(1).ok_or_else(or_else)?.as_str();
            let webdriver_version = lenient_semver::parse(version_str)
                .map_err(|e| VersionReqError::ParseVersion(e.owned()))?;

            let version_req = VersionReq::parse(&format!("^{}", webdriver_version))
                .map_err(VersionReqError::ParseVersionReq)?;

            versions.push(WebdriverVersionUrl {
                version_req,
                webdriver_version,
                url: os_specific::chromedriver::build_url(version_str),
            });
        }

        Ok(versions)
    }
}

impl WebdriverInstallationInfo for ChromedriverInfo {
    fn driver_install_path(&self) -> &Path {
        &self.driver_install_path
    }

    fn driver_executable_name(&self) -> &'static str {
        os_specific::chromedriver::DRIVER_EXECUTABLE_NAME
    }
}

impl WebdriverVerificationInfo for ChromedriverInfo {
    fn driver_capabilities(&self) -> Option<Capabilities> {
        let capabilities_value = json!({
            "binary": self.browser_path,
            "args": ["-headless"],
        });

        let mut capabilities = Map::new();

        capabilities.insert("goog:chromeOptions".to_string(), capabilities_value);

        Some(capabilities)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use super::ChromedriverInfo;

    #[test]
    fn test_get_binary_version() {
        let browser_path = os_specific::chromedriver::default_browser_path()
            .expect("Failed to get default browser path");

        let chromedriver_info = ChromedriverInfo {
            driver_install_path: "".into(),
            browser_path,
        };

        assert!(chromedriver_info.binary_version().is_ok());
    }
}
