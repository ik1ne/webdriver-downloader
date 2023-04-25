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

/// Information required to implement [WebdriverDownloadInfo](crate::WebdriverDownloadInfo) for Geckodriver.
pub struct GeckodriverInfo {
    driver_install_path: PathBuf,
    browser_path: PathBuf,
}

impl GeckodriverInfo {
    pub fn new(driver_install_path: PathBuf, browser_path: PathBuf) -> Self {
        GeckodriverInfo {
            driver_install_path,
            browser_path,
        }
    }

    pub fn new_default() -> Result<Self, DefaultPathError> {
        let driver_install_path = os_specific::geckodriver::default_driver_path()?;
        let browser_path = os_specific::geckodriver::default_browser_path()?;
        Ok(GeckodriverInfo::new(driver_install_path, browser_path))
    }
}

#[async_trait]
impl VersionReqUrlInfo for GeckodriverInfo {
    fn binary_version(&self) -> Result<Version, VersionReqError> {
        os_specific::geckodriver::binary_version(&self.browser_path)
    }

    async fn driver_version_urls(&self) -> Result<Vec<WebdriverVersionUrl>, UrlError> {
        let download_html =
            "https://firefox-source-docs.mozilla.org/_sources/testing/geckodriver/Support.md.txt";

        let html = reqwest::get(download_html).await?.text().await?;

        let html = html
            .lines()
            .skip_while(|line| *line != "<table>")
            .take_while(|line| !line.is_empty())
            .collect::<String>();

        // parses <tr>  <td>0.33.0  <td>≥ 3.11 (3.14 Python)  <td>102 ESR  <td>n/a
        // or <tr>  <td>0.19.0  <td>≥ 3.5  <td>55  <td>62
        let re = Regex::new(
            r#"<tr>\s*<td>([0-9.]*?)\s*<td>[^<]*<td>([0-9.]*?)( ESR)?\s*<td>([0-9.]*|n/a)"#,
        )
        .unwrap();

        let mut versions: Vec<WebdriverVersionUrl> = vec![];
        for captures in re.captures_iter(&html) {
            let or_else =
                || VersionReqError::RegexError(captures.get(0).unwrap().as_str().to_string());

            let version_str = captures.get(1).ok_or_else(or_else)?.as_str();
            let min_firefox_version_str = captures.get(2).ok_or_else(or_else)?.as_str();
            let max_firefox_version_str = captures.get(4).ok_or_else(or_else)?.as_str();

            let webdriver_version = lenient_semver::parse(version_str)
                .map_err(|e| VersionReqError::ParseVersion(e.owned()))?;
            let min_version = lenient_semver::parse(min_firefox_version_str)
                .map_err(|e| VersionReqError::ParseVersion(e.owned()))?;
            let max_version = lenient_semver::parse(max_firefox_version_str).ok();

            let version_req_string = match max_version {
                Some(max_version) => format!(">= {}, <= {}", min_version, max_version),
                None => format!(">= {}", min_version),
            };

            let version_req =
                VersionReq::parse(&version_req_string).map_err(VersionReqError::ParseVersionReq)?;

            versions.push(WebdriverVersionUrl {
                version_req,
                webdriver_version,
                url: os_specific::geckodriver::build_url(version_str),
            })
        }

        Ok(versions)
    }
}

impl WebdriverInstallationInfo for GeckodriverInfo {
    fn driver_install_path(&self) -> &Path {
        &self.driver_install_path
    }

    fn driver_executable_name(&self) -> &'static str {
        os_specific::geckodriver::DRIVER_EXECUTABLE_NAME
    }
}

impl WebdriverVerificationInfo for GeckodriverInfo {
    fn driver_capabilities(&self) -> Option<Capabilities> {
        let capabilities_value = json!({
            "binary": self.browser_path,
            "args": ["-headless"],
        });

        let mut capabilities = Map::new();

        capabilities.insert("moz:firefoxOptions".to_string(), capabilities_value);

        Some(capabilities)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use super::GeckodriverInfo;

    #[test]
    fn test_get_binary_version() {
        let browser_path = os_specific::geckodriver::default_browser_path()
            .expect("Failed to get default browser path");

        let geckodriver_info = GeckodriverInfo {
            driver_install_path: "".into(),
            browser_path,
        };

        assert!(geckodriver_info.binary_version().is_ok());
    }
}
