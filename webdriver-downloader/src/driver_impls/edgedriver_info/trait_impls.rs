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

use super::EdgedriverInfo;

#[async_trait]
impl VersionReqUrlInfo for EdgedriverInfo {
    fn binary_version(&self) -> Result<Version, VersionReqError> {
        os_specific::edgedriver::binary_version(&self.browser_path)
    }

    async fn driver_version_urls(&self) -> Result<Vec<WebdriverVersionUrl>, UrlError> {
        // For EdgeDriver, we need to match the browser version exactly
        // EdgeDriver follows the same version as the Edge browser
        let browser_version = self
            .binary_version()
            .map_err(|e| UrlError::Other(e.into()))?;

        // Get the original full version string for the URL (with 4 components)
        let original_version = os_specific::edgedriver::binary_version_string(&self.browser_path)
            .map_err(|e| UrlError::Other(e.into()))?;

        let url = os_specific::edgedriver::build_url(&original_version);

        // EdgeDriver version must match browser version exactly
        let version_req = format!("={}", browser_version)
            .parse()
            .map_err(|e: semver::Error| UrlError::Other(e.into()))?;

        Ok(vec![WebdriverVersionUrl {
            version_req,
            webdriver_version: browser_version,
            url,
        }])
    }
}

impl WebdriverInstallationInfo for EdgedriverInfo {
    fn driver_install_path(&self) -> &Path {
        &self.driver_install_path
    }

    fn driver_executable_name(&self) -> &'static str {
        os_specific::edgedriver::DRIVER_EXECUTABLE_NAME
    }
}

impl WebdriverVerificationInfo for EdgedriverInfo {
    fn driver_capabilities(&self) -> Option<Capabilities> {
        let capabilities_value = json!({
            "binary": self.browser_path,
            "args": ["--headless"],
            "useAutomationExtension": false,
            "excludeSwitches": ["enable-automation"],
        });

        let mut capabilities = Map::new();

        // Use Microsoft Edge-specific capability key
        capabilities.insert("ms:edgeOptions".to_string(), capabilities_value);

        Some(capabilities)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use std::path::PathBuf;
    use test_log::test;

    use crate::prelude::EdgedriverInfo;
    use crate::prelude::*;

    #[test]
    fn test_get_binary_version() -> Result<()> {
        let browser_path = os_specific::edgedriver::default_browser_path()
            .expect("Failed to get default browser path");

        let edgedriver_info = EdgedriverInfo {
            driver_install_path: "".into(),
            browser_path,
        };

        // This test will only pass if Microsoft Edge is installed
        if edgedriver_info.browser_path.exists() {
            edgedriver_info.binary_version()?;
        }

        Ok(())
    }

    #[test]
    fn test_driver_executable_name() {
        let edgedriver_info = EdgedriverInfo {
            driver_install_path: PathBuf::from("/tmp/test"),
            browser_path: PathBuf::from("/tmp/edge"),
        };

        #[cfg(target_os = "windows")]
        assert_eq!(edgedriver_info.driver_executable_name(), "msedgedriver.exe");

        #[cfg(not(target_os = "windows"))]
        assert_eq!(edgedriver_info.driver_executable_name(), "msedgedriver");
    }

    #[test]
    fn test_driver_capabilities() {
        let browser_path =
            PathBuf::from("/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge");
        let edgedriver_info = EdgedriverInfo {
            driver_install_path: PathBuf::from("/tmp/test"),
            browser_path: browser_path.clone(),
        };

        let capabilities = edgedriver_info.driver_capabilities();
        assert!(capabilities.is_some());

        let caps = capabilities.unwrap();
        assert!(caps.contains_key("ms:edgeOptions"));

        let edge_options = &caps["ms:edgeOptions"];
        assert_eq!(
            edge_options["binary"],
            serde_json::Value::String(browser_path.to_string_lossy().to_string())
        );
    }

    #[test]
    fn test_build_url_format() {
        let version = "141.0.3537.57";
        let url = os_specific::edgedriver::build_url(version);

        assert!(url.starts_with("https://msedgedriver.microsoft.com/"));
        assert!(url.contains(version));
        assert!(url.ends_with(".zip"));

        // Check platform-specific URL suffix
        #[cfg(all(target_os = "windows", target_arch = "x86"))]
        assert!(url.contains("edgedriver_win32.zip"));

        #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
        assert!(url.contains("edgedriver_win64.zip"));

        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        assert!(url.contains("edgedriver_mac64.zip"));

        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        assert!(url.contains("edgedriver_mac64_m1.zip"));

        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        assert!(url.contains("edgedriver_linux64.zip"));
    }
}
