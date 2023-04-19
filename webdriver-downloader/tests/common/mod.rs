use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use anyhow::anyhow;
use async_trait::async_trait;
use fantoccini::wd::Capabilities;
use reqwest::IntoUrl;
use tempfile::TempDir;

use webdriver_downloader::installation_info::{InstallationError, WebdriverInstallationInfo};
use webdriver_downloader::url_info::{UrlError, WebdriverUrlInfo, WebdriverVersionUrl};
use webdriver_downloader::verification_info::{VerificationError, WebdriverVerificationInfo};

#[derive(Debug)]
pub struct MockWebdriverDownloadInfo<'a> {
    // url_info
    pub version_urls: Option<Vec<WebdriverVersionUrl>>,

    // installation_info
    pub driver_install_path: &'a Path,
    pub driver_name_in_archive: &'a str,
    pub download_in_tempdir: Option<PathBuf>,
    pub install_driver: Arc<Mutex<Vec<bool>>>,

    // verification_info
    pub driver_capabilities: Option<Capabilities>,
    pub verify_driver: Arc<Mutex<Vec<bool>>>,
}

impl<'a> MockWebdriverDownloadInfo<'a> {
    pub fn new() -> Self {
        MockWebdriverDownloadInfo {
            version_urls: Default::default(),
            driver_install_path: Path::new(""),
            driver_name_in_archive: Default::default(),
            download_in_tempdir: Default::default(),
            install_driver: Default::default(),
            driver_capabilities: Default::default(),
            verify_driver: Default::default(),
        }
    }
}

#[async_trait]
impl WebdriverUrlInfo for MockWebdriverDownloadInfo<'_> {
    async fn version_urls(&self, _limit: usize) -> Result<Vec<WebdriverVersionUrl>, UrlError> {
        match &self.version_urls {
            None => return Err(anyhow!("error").into()),
            Some(v) => Ok(v.clone()),
        }
    }
}

#[async_trait]
impl WebdriverInstallationInfo for MockWebdriverDownloadInfo<'_> {
    fn driver_install_path(&self) -> &Path {
        self.driver_install_path
    }

    fn driver_name_in_archive(&self) -> &str {
        self.driver_name_in_archive
    }

    async fn download_in_tempdir<U: IntoUrl + AsRef<str> + Send>(
        &self,
        _url: U,
        _dir: &TempDir,
    ) -> Result<PathBuf, InstallationError> {
        self.download_in_tempdir
            .clone()
            .ok_or(anyhow!("error").into())
    }

    fn install_driver<P: AsRef<Path>>(
        &self,
        _temp_driver_path: &P,
    ) -> Result<(), InstallationError> {
        match self.install_driver.lock().unwrap().remove(0) {
            true => Ok(()),
            false => Err(anyhow!("error").into()),
        }
    }
}

#[async_trait]
impl WebdriverVerificationInfo for MockWebdriverDownloadInfo<'_> {
    fn driver_capabilities(&self) -> Option<Capabilities> {
        self.driver_capabilities.clone()
    }

    async fn verify_driver<P: AsRef<Path> + Sync>(
        &self,
        _driver_path: &P,
    ) -> Result<(), VerificationError> {
        match self.verify_driver.lock().unwrap().remove(0) {
            true => Ok(()),
            false => Err(anyhow!("error").into()),
        }
    }

    async fn test_client(_client: &fantoccini::Client) -> Result<(), VerificationError> {
        todo!()
    }
}
