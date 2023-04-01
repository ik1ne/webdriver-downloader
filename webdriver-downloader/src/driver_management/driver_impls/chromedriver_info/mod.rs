use std::path::{Path, PathBuf};

use anyhow::Context;
use async_trait::async_trait;
use fantoccini::wd::Capabilities;
use regex::Regex;
use semver::VersionReq;
use serde_json::{json, Map};

use crate::common::version_req_url_info::{VersionReqError, VersionReqUrlInfo};
use crate::common::installation_info::WebdriverInstallationInfo;
use crate::common::url_info::{UrlError, VersionUrl};
use crate::common::verification_info::WebdriverVerificationInfo;

mod os_specific;

/// Information required to implement [WebdriverInfo](crate::WebdriverInfo) for Chromedriver.
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
}

#[async_trait]
impl VersionReqUrlInfo for ChromedriverInfo {
    fn version_req(&self) -> Result<VersionReq, VersionReqError> {
        let version_string = format!("^{}", os_specific::binary_version(&self.browser_path)?);
        VersionReq::parse(&version_string).map_err(|e| e.into())
    }

    async fn driver_version_urls(&self) -> Result<Vec<VersionUrl>, UrlError> {
        let download_xml = "https://chromedriver.storage.googleapis.com";

        let xml = reqwest::get(download_xml).await?.text().await?;

        let re = Regex::new(os_specific::ZIPFILE_NAME_RE).expect("Failed to parse regex.");

        let mut versions = vec![];
        for capture in re.captures_iter(&xml) {
            let version_string = capture.get(1).map_or("", |s| s.as_str()).to_string();
            let version = lenient_semver::parse(&version_string)
                .map_err(|e| e.owned())
                .with_context(|| format!("Failed to parse version: \"{}\"", version_string))?;

            versions.push((version_string, version));
        }

        versions.sort_by(|l, r| l.1.cmp(&r.1).reverse());

        Ok(versions
            .into_iter()
            .map(|(version_string, version)| VersionUrl {
                version,
                url: os_specific::build_url(&version_string),
            })
            .collect())
    }
}

impl WebdriverInstallationInfo for ChromedriverInfo {
    fn driver_install_path(&self) -> &Path {
        &self.driver_install_path
    }

    fn driver_name_in_archive(&self) -> &'static str {
        os_specific::DRIVER_NAME_IN_ARCHIVE
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
    use crate::common::version_req_url_info::VersionReqUrlInfo;
    use crate::driver_impls::ChromedriverInfo;

    #[test]
    fn test_get_binary_version() {
        #[cfg(target_os = "windows")]
        let browser_path = r"C:\Program Files\Google\Chrome\Application\chrome.exe";
        #[cfg(target_os = "linux")]
        let browser_path = "/usr/bin/google-chrome";
        #[cfg(target_os = "macos")]
        let browser_path = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";

        let chromedriver_info = ChromedriverInfo {
            driver_install_path: "".into(),
            browser_path: browser_path.into(),
        };

        assert!(chromedriver_info.version_req().is_ok());
    }
}
