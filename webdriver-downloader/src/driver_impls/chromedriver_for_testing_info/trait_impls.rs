use std::collections::HashMap;
use std::path::Path;

use async_trait::async_trait;
use fantoccini::wd::Capabilities;
use semver::VersionReq;
use serde::Deserialize;
use serde_json::{json, Map};

use crate::os_specific;
use crate::prelude::{
    UrlError, VersionReqError, VersionReqUrlInfo, WebdriverInstallationInfo,
    WebdriverVerificationInfo, WebdriverVersionUrl,
};

use super::ChromedriverForTestingInfo;

/// struct to parse the json from version url.
#[derive(Debug, Deserialize)]
struct JsonRoot {
    // timestamp: String,
    versions: Vec<Version>,
}

/// struct to parse the json from version url.
/// Do not get confused with the `Version` struct from `semver` crate.
#[derive(Debug, Deserialize)]
struct Version {
    version: String,
    // revision: String,
    downloads: HashMap<String, Vec<Download>>,
}

#[derive(Debug, Deserialize)]
struct Download {
    platform: String,
    url: String,
}

#[async_trait]
impl VersionReqUrlInfo for ChromedriverForTestingInfo {
    fn binary_version(&self) -> Result<semver::Version, VersionReqError> {
        os_specific::chromedriver_for_testing::binary_version(&self.browser_path)
    }

    async fn driver_version_urls(&self) -> Result<Vec<WebdriverVersionUrl>, UrlError> {
        let response = reqwest::get("https://googlechromelabs.github.io/chrome-for-testing/known-good-versions-with-downloads.json")
        .await?
        .text()
        .await?;

        let deserialized: JsonRoot = serde_json::from_str(&response)?;

        let versions = deserialized.versions;

        versions
            .into_iter()
            .map(version_into_webdriver_version_url)
            .filter_map(|x| x.transpose())
            .collect::<Result<Vec<_>, UrlError>>()
    }
}

/// Converts a `Version` struct into a `WebdriverVersionUrl` struct.
/// Since there are cases where chromedriver url does not exist, this function returns `Ok(None)` if the url does not exist.
fn version_into_webdriver_version_url(
    mut version: Version,
) -> Result<Option<WebdriverVersionUrl>, UrlError> {
    let version_str = version.version;
    let webdriver_version = lenient_semver::parse(&version_str)
        .map_err(|e| VersionReqError::ParseVersion(e.owned()))?;
    let version_req = VersionReq::parse(&format!("^{}", webdriver_version))
        .map_err(VersionReqError::ParseVersionReq)?;
    let Some(downloads) = version.downloads.remove("chromedriver") else {
        return Ok(None);
    };

    let url = downloads
        .into_iter()
        .filter(|download| download.platform == os_specific::chromedriver_for_testing::PLATFORM)
        .map(|download| download.url)
        .next();

    let Some(url) = url else {
        return Ok(None);
    };

    Ok(Some(WebdriverVersionUrl {
        version_req,
        webdriver_version,
        url,
    }))
}

impl WebdriverInstallationInfo for ChromedriverForTestingInfo {
    fn driver_install_path(&self) -> &Path {
        &self.driver_install_path
    }

    fn driver_executable_name(&self) -> &'static str {
        os_specific::chromedriver_old::DRIVER_EXECUTABLE_NAME
    }
}

impl WebdriverVerificationInfo for ChromedriverForTestingInfo {
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

    use super::ChromedriverForTestingInfo;

    #[test]
    fn test_get_binary_version() {
        let browser_path = os_specific::chromedriver_for_testing::default_browser_path()
            .expect("Failed to get default browser path");

        let chromedriver_info = ChromedriverForTestingInfo {
            driver_install_path: "".into(),
            browser_path,
        };

        assert!(chromedriver_info.binary_version().is_ok());
    }
}
