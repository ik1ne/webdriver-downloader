use std::path::Path;

use async_trait::async_trait;
use fantoccini::wd::Capabilities;
use semver::Version;
use serde_json::{json, Map};

use crate::os_specific;
use crate::traits::installation_info::WebdriverInstallationInfo;
use crate::traits::url_info::{UrlError, WebdriverVersionUrl};
use crate::traits::verification_info::WebdriverVerificationInfo;
use crate::traits::version_req_url_info::{VersionReqError, VersionReqUrlInfo};

use super::ChromedriverInfo;

#[async_trait]
impl VersionReqUrlInfo for ChromedriverInfo {
    fn binary_version(&self) -> Result<Version, VersionReqError> {
        let browser_path = match self {
            ChromedriverInfo::OldInfo(old_info) => &old_info.browser_path,
            ChromedriverInfo::NewInfo(new_info) => &new_info.browser_path,
        };

        os_specific::chromedriver::binary_version(browser_path)
    }

    async fn driver_version_urls(&self) -> Result<Vec<WebdriverVersionUrl>, UrlError> {
        match self {
            ChromedriverInfo::OldInfo(old_info) => old_info.driver_version_urls().await,
            ChromedriverInfo::NewInfo(new_info) => new_info.driver_version_urls().await,
        }
    }
}

impl WebdriverInstallationInfo for ChromedriverInfo {
    fn driver_install_path(&self) -> &Path {
        match self {
            ChromedriverInfo::OldInfo(old_info) => old_info.driver_install_path(),
            ChromedriverInfo::NewInfo(new_info) => new_info.driver_install_path(),
        }
    }

    fn driver_executable_name(&self) -> &'static str {
        os_specific::chromedriver::DRIVER_EXECUTABLE_NAME
    }
}

impl WebdriverVerificationInfo for ChromedriverInfo {
    fn driver_capabilities(&self) -> Option<Capabilities> {
        match self {
            ChromedriverInfo::OldInfo(old_info) => old_info.driver_capabilities(),
            ChromedriverInfo::NewInfo(new_info) => new_info.driver_capabilities(),
        }
    }
}
