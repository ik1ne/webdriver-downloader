use std::path::{Path, PathBuf};

use async_trait::async_trait;
use fantoccini::wd::Capabilities;
use mockall::mock;
use reqwest::IntoUrl;
use tempfile::TempDir;

use webdriver_downloader::installation_info::{InstallationError, WebdriverInstallationInfo};
use webdriver_downloader::url_info::{UrlError, WebdriverUrlInfo, WebdriverVersionUrl};
use webdriver_downloader::verification_info::{VerificationError, WebdriverVerificationInfo};

mod driver_management;

mock! {
    WebdriverInfo {}


    #[async_trait]
    impl WebdriverUrlInfo for WebdriverInfo {
        async fn version_urls(&self, limit: usize) -> Result<Vec<WebdriverVersionUrl>, UrlError>;
    }

    #[async_trait]
    impl WebdriverInstallationInfo for WebdriverInfo {
        fn driver_install_path(&self) -> &Path;

        fn driver_name_in_archive(&self) -> &'static str;

        async fn download_in_tempdir<U: 'static + IntoUrl + Send>(
            &self,
            url: U,
            dir: &TempDir,
        ) -> Result<PathBuf, InstallationError>;

        fn install_driver<P: AsRef<Path> +'static>(
            &self,
            driver_path: &P,
        ) -> Result<(), InstallationError>;
    }

    #[async_trait]
    impl WebdriverVerificationInfo for WebdriverInfo {
        fn driver_capabilities(&self) -> Option<Capabilities>;

        async fn verify_driver<P: AsRef<Path> + Sync + 'static>(
            &self,
            driver_path: &P,
        ) -> Result<(), VerificationError>;

        async fn test_client(client: &fantoccini::Client) -> Result<(), VerificationError>;
    }
}
