use async_trait::async_trait;
use fantoccini::wd::Capabilities;
use serde_json::json;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

use crate::driver_management::url_info::VersionUrl;
use crate::{WebdriverInstallationInfo, WebdriverUrlInfo, WebdriverVerificationInfo};

pub struct ChromedriverInfo {
    driver_install_path: PathBuf,
    browser_path: PathBuf,
    temp_dir: TempDir,
}

impl ChromedriverInfo {
    pub fn new(driver_install_path: PathBuf, browser_path: PathBuf, temp_dir: TempDir) -> Self {
        ChromedriverInfo {
            driver_install_path,
            browser_path,
            temp_dir,
        }
    }
}

#[async_trait]
impl WebdriverUrlInfo for ChromedriverInfo {
    fn driver_version_hint(&self) -> Option<&OsStr> {
        todo!()
    }

    async fn driver_url_infos(&self) -> anyhow::Result<Vec<VersionUrl>> {
        todo!()
    }
}

impl WebdriverInstallationInfo for ChromedriverInfo {
    fn driver_install_path(&self) -> &Path {
        todo!()
    }

    fn temp_dir_path(&self) -> &Path {
        todo!()
    }

    fn driver_name_in_zip(&self) -> &'static str {
        "chromedriver.exe"
    }
}

impl WebdriverVerificationInfo for ChromedriverInfo {
    fn browser_path(&self) -> &Path {
        todo!()
    }

    fn driver_path(&self) -> &Path {
        todo!()
    }

    fn driver_capabilities(&self) -> Capabilities {
        let capabilities_value = json!( {
            "goog:chromeOption":  {
                "binary": r"C:\Program Files\Google\Chrome Dev\Application\chrome.exe"
            }
        });

        if let serde_json::Value::Object(capabilities) = capabilities_value {
            capabilities
        } else {
            panic!("Failed to construct capabilities")
        }
    }
}
