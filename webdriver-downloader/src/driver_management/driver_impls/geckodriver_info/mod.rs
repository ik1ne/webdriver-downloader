use std::path::{Path, PathBuf};

use async_trait::async_trait;
use fantoccini::wd::Capabilities;
use scraper::{Html, Selector};
use semver::{Version, VersionReq};
use serde_json::{json, Map};

use crate::common::installation_info::WebdriverInstallationInfo;
use crate::common::url_info::{UrlError, WebdriverVersionUrl};
use crate::common::verification_info::WebdriverVerificationInfo;
use crate::common::version_req_url_info::{VersionReqError, VersionReqUrlInfo};

mod os_specific;

/// Information required to implement [WebdriverInfo](crate::WebdriverInfo) for Geckodriver.
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
}

#[async_trait]
impl VersionReqUrlInfo for GeckodriverInfo {
    fn binary_version(&self) -> Result<Version, VersionReqError> {
        os_specific::binary_version(&self.browser_path)
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

        let document = Html::parse_fragment(&html);

        if !document.errors.is_empty() {
            return Err(UrlError::Parse(html));
        }

        let mut results = vec![];

        let row_selector = Selector::parse("table tr").unwrap();
        let cell_selector = Selector::parse("td").unwrap();

        for row in document.select(&row_selector).skip(1) {
            let mut cells = row.select(&cell_selector);
            if let (Some(geckodriver_version), Some(firefox_version)) = (cells.next(), cells.nth(1))
            {
                let geckodriver_version_string = geckodriver_version
                    .text()
                    .collect::<String>()
                    .chars()
                    .take_while(|c| *c != ' ')
                    .collect::<String>();
                let firefox_version = firefox_version
                    .text()
                    .collect::<String>()
                    .chars()
                    .take_while(|c| *c != ' ')
                    .collect::<String>();

                let firefox_version_req =
                    VersionReq::parse(&format!(">={}", firefox_version)).unwrap();

                let geckodriver_version =
                    lenient_semver::parse(&geckodriver_version_string).map_err(|e| e.owned())?;

                results.push(WebdriverVersionUrl {
                    version_req: firefox_version_req,
                    webdriver_version: geckodriver_version,
                    url: os_specific::build_url(&geckodriver_version_string),
                })
            }
        }

        Ok(results)
    }
}

impl WebdriverInstallationInfo for GeckodriverInfo {
    fn driver_install_path(&self) -> &Path {
        &self.driver_install_path
    }

    fn driver_name_in_archive(&self) -> &'static str {
        os_specific::DRIVER_NAME_IN_ARCHIVE
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
    use crate::common::version_req_url_info::VersionReqUrlInfo;
    use crate::driver_impls::GeckodriverInfo;

    #[test]
    fn test_get_binary_version() {
        #[cfg(target_os = "windows")]
        let browser_path = r"C:\Program Files\Mozilla Firefox\firefox.exe";
        #[cfg(target_os = "linux")]
        let browser_path = "/usr/bin/firefox";
        #[cfg(target_os = "macos")]
        let browser_path = "/Applications/Firefox.app/Contents/MacOS/firefox";

        let geckodriver_info = GeckodriverInfo {
            driver_install_path: "".into(),
            browser_path: browser_path.into(),
        };

        assert!(geckodriver_info.binary_version().is_ok());
    }
}
