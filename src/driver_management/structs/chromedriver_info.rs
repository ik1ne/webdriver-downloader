use std::borrow::Borrow;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use anyhow::Result;
use async_trait::async_trait;
use fantoccini::wd::Capabilities;
use regex::Regex;
use semver::Version;
use serde_json::json;

use crate::driver_management::traits::{BinaryMajorVersionHintUrlInfo, VersionUrl};
use crate::{WebdriverInstallationInfo, WebdriverVerificationInfo};

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
impl BinaryMajorVersionHintUrlInfo for ChromedriverInfo {
    fn binary_version(&self) -> Option<Version> {
        let mut child = std::process::Command::new("powershell");

        let mut command = OsString::from("(Get-Item \"");
        command.push(&self.browser_path);
        command.push("\").VersionInfo.FileVersion");

        child.arg("-command").arg(command);

        let output = child.output().ok()?;
        lenient_semver::parse(String::from_utf8_lossy(&output.stdout).borrow()).ok()
    }

    async fn driver_version_urls(&self) -> Result<Vec<VersionUrl>> {
        let download_xml = "https://chromedriver.storage.googleapis.com";

        let xml = reqwest::get(download_xml).await?.text().await?;

        let re =
            Regex::new(r"<Key>(.*?)/chromedriver_win32.zip</Key>").expect("Failed to parse regex.");

        let mut versions: Vec<_> = re
            .captures_iter(&xml)
            .filter_map(|x| Some((x[1].to_owned(), lenient_semver::parse(&x[1]).ok()?)))
            .collect();

        versions.sort_by(|l, r| l.1.cmp(&r.1).reverse());

        Ok(versions
            .into_iter()
            .map(|(version_string, version)| VersionUrl {
                url: format!(
                    "https://chromedriver.storage.googleapis.com/{}/chromedriver_win32.zip",
                    version_string
                ),
                driver_version: version,
            })
            .collect())
    }
}

impl WebdriverInstallationInfo for ChromedriverInfo {
    fn driver_install_path(&self) -> &Path {
        &self.driver_install_path
    }

    fn driver_name_in_archive(&self) -> &'static str {
        "chromedriver.exe"
    }
}

impl WebdriverVerificationInfo for ChromedriverInfo {
    fn driver_capabilities(&self) -> Option<Capabilities> {
        let capabilities_value = json!( {
            "goog:chromeOptions":  {
                "binary": self.browser_path
            }
        });

        if let serde_json::Value::Object(capabilities) = capabilities_value {
            Some(capabilities)
        } else {
            panic!("Failed to construct capabilities")
        }
    }
}
