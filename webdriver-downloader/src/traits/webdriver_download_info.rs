use std::io;

use async_trait::async_trait;
use tempfile::TempDir;

use crate::os_specific::DefaultPathError;
use crate::traits::installation_info::{InstallationError, WebdriverInstallationInfo};
use crate::traits::url_info::{UrlError, WebdriverUrlInfo};
use crate::traits::verification_info::{VerificationError, WebdriverVerificationInfo};

mod sealed {
    pub trait Sealed {}
}

impl<T> sealed::Sealed for T where
    T: WebdriverUrlInfo + WebdriverInstallationInfo + WebdriverVerificationInfo + Sync
{
}

/// Downloads, verifies and installs a driver.
///
/// This trait is implemented for the drivers in [`crate::driver_impls`] and can't be implemented
/// outside this crate:
///
/// ```compile_fail
/// use webdriver_downloader::prelude::*;
///
/// struct MyDriver;
///
/// #[async_trait::async_trait]
/// impl WebdriverDownloadInfo for MyDriver {
///     async fn is_installed(&self) -> bool {
///         false
///     }
///
///     async fn download_install(&self) -> Result<(), WebdriverDownloadError> {
///         Ok(())
///     }
///
///     async fn download_verify_install(&self, _: usize) -> Result<(), WebdriverDownloadError> {
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait WebdriverDownloadInfo: sealed::Sealed + Sync {
    /// Check if the driver is installed.
    async fn is_installed(&self) -> bool;

    /// Download, install driver. Skips verification process.
    async fn download_install(&self) -> Result<(), WebdriverDownloadError>;

    /// Download, verify, install driver.
    async fn download_verify_install(&self, max_tries: usize)
        -> Result<(), WebdriverDownloadError>;
}

/// Error that can occur when installing and verifying driver.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum WebdriverDownloadError {
    #[error(transparent)]
    Url(#[from] UrlError),
    #[error(transparent)]
    Install(#[from] InstallationError),
    #[error(transparent)]
    Verify(#[from] VerificationError),
    #[error(transparent)]
    DefaultPath(#[from] DefaultPathError),
    #[error("Failed to move driver to driver_path: {0}")]
    Move(#[from] io::Error),
    #[error("Tried {0} possible versions, but no version passed verification.")]
    NoVersionPassedVerification(usize),
}

#[async_trait]
impl<T> WebdriverDownloadInfo for T
where
    T: WebdriverUrlInfo + WebdriverInstallationInfo + WebdriverVerificationInfo + Sync,
{
    async fn is_installed(&self) -> bool {
        let driver_path = self.driver_install_path();
        self.verify_driver(&driver_path).await.is_ok()
    }

    async fn download_install(&self) -> Result<(), WebdriverDownloadError> {
        let mut version_urls = self.version_urls(1).await?;
        let version_url = version_urls
            .pop()
            .ok_or(WebdriverDownloadError::NoVersionPassedVerification(0))?;

        println!(
            "Trying url for version {}: {}.",
            version_url.webdriver_version, version_url.url
        );

        let tempdir = TempDir::new()?;

        let temp_driver_path = self.download_in_tempdir(version_url.url, &tempdir).await?;

        self.install_driver(&temp_driver_path)?;

        Ok(())
    }

    async fn download_verify_install(
        &self,
        max_tries: usize,
    ) -> Result<(), WebdriverDownloadError> {
        let version_urls = self.version_urls(max_tries).await?;
        let url_count = version_urls.len();

        for version_url in version_urls {
            println!(
                "Trying url for version {}: {}.",
                version_url.webdriver_version, version_url.url
            );
            let tempdir = TempDir::new()?;

            let temp_driver_path = self.download_in_tempdir(version_url.url, &tempdir).await?;

            match self.verify_driver(&temp_driver_path).await {
                Ok(_) => {
                    self.install_driver(&temp_driver_path)?;
                    return Ok(());
                }
                Err(e) => {
                    println!("Verification failed: {}.", e)
                }
            }
        }

        Err(WebdriverDownloadError::NoVersionPassedVerification(
            url_count,
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use std::sync::{Arc, Mutex};

    use anyhow::Result;
    use async_trait::async_trait;
    use fantoccini::wd::Capabilities;
    use reqwest::IntoUrl;
    use semver::Version;
    use tempfile::TempDir;

    use crate::traits::installation_info::{InstallationError, WebdriverInstallationInfo};
    use crate::traits::url_info::{UrlError, WebdriverUrlInfo, WebdriverVersionUrl};
    use crate::traits::verification_info::{VerificationError, WebdriverVerificationInfo};

    use super::WebdriverDownloadInfo;

    #[derive(Debug)]
    struct MockWebdriverDownloadInfo<'a> {
        // url_info
        version_urls: Option<Vec<WebdriverVersionUrl>>,

        // installation_info
        driver_install_path: &'a Path,
        driver_executable_name: &'a str,
        download_in_tempdir: Option<PathBuf>,
        install_driver: Arc<Mutex<Vec<bool>>>,

        // verification_info
        driver_capabilities: Option<Capabilities>,
        verify_driver: Arc<Mutex<Vec<bool>>>,
    }

    impl MockWebdriverDownloadInfo<'_> {
        fn new() -> Self {
            MockWebdriverDownloadInfo {
                version_urls: Default::default(),
                driver_install_path: Path::new(""),
                driver_executable_name: Default::default(),
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
                None => Err(UrlError::Parse("error".to_string())),
                Some(v) => Ok(v.clone()),
            }
        }
    }

    #[async_trait]
    impl WebdriverInstallationInfo for MockWebdriverDownloadInfo<'_> {
        fn driver_install_path(&self) -> &Path {
            self.driver_install_path
        }

        fn driver_executable_name(&self) -> &str {
            self.driver_executable_name
        }

        async fn download_in_tempdir<U: IntoUrl + AsRef<str> + std::fmt::Debug + Send>(
            &self,
            _url: U,
            _dir: &TempDir,
        ) -> Result<PathBuf, InstallationError> {
            self.download_in_tempdir
                .clone()
                .ok_or(InstallationError::UnknownArchiveFormat)
        }

        fn install_driver<P: AsRef<Path> + std::fmt::Debug>(
            &self,
            _temp_driver_path: &P,
        ) -> Result<(), InstallationError> {
            match self.install_driver.lock().unwrap().remove(0) {
                true => Ok(()),
                false => Err(InstallationError::UnknownArchiveFormat),
            }
        }
    }

    #[async_trait]
    impl WebdriverVerificationInfo for MockWebdriverDownloadInfo<'_> {
        fn driver_capabilities(&self) -> Option<Capabilities> {
            self.driver_capabilities.clone()
        }

        async fn verify_driver<P: AsRef<Path> + std::fmt::Debug + Sync>(
            &self,
            _driver_path: &P,
        ) -> Result<(), VerificationError> {
            match self.verify_driver.lock().unwrap().remove(0) {
                true => Ok(()),
                false => Err(VerificationError::Start(std::io::Error::other("error"))),
            }
        }

        async fn test_client(_client: &fantoccini::Client) -> Result<(), VerificationError> {
            todo!()
        }
    }

    fn dummy_urls(count: usize) -> Vec<WebdriverVersionUrl> {
        let dummy_version_url = WebdriverVersionUrl {
            version_req: Default::default(),
            webdriver_version: Version::new(0, 0, 0),
            url: Default::default(),
        };
        vec![dummy_version_url; count]
    }

    #[tokio::test]
    async fn fails_when_0_max_tries() {
        let mut mock = MockWebdriverDownloadInfo::new();
        mock.version_urls = Some(vec![]);

        let result = mock.download_verify_install(0).await;
        result.unwrap_err();
    }

    #[tokio::test]
    async fn passes_when_one_version_passes() -> Result<()> {
        let mut mock = MockWebdriverDownloadInfo::new();
        let version_count = 5;

        mock.version_urls = Some(dummy_urls(version_count));
        mock.download_in_tempdir = Some(Default::default());
        mock.verify_driver = Arc::new(Mutex::new(vec![false, false, false, false, true]));
        mock.install_driver = Arc::new(Mutex::new(vec![true; version_count]));

        mock.download_verify_install(version_count).await?;

        Ok(())
    }

    #[tokio::test]
    async fn fails_when_all_versions_fail() {
        let mut mock = MockWebdriverDownloadInfo::new();
        let version_count = 5;

        mock.version_urls = Some(dummy_urls(version_count));
        mock.download_in_tempdir = Some(Default::default());
        mock.verify_driver = Arc::new(Mutex::new(vec![false; 5]));

        let result = mock.download_verify_install(version_count).await;
        result.unwrap_err();
    }

    #[tokio::test]
    async fn fails_when_installation_fails() {
        let mut mock = MockWebdriverDownloadInfo::new();
        let version_count = 5;

        mock.version_urls = Some(dummy_urls(version_count));
        mock.download_in_tempdir = Some(Default::default());
        mock.verify_driver = Arc::new(Mutex::new(vec![true; 5]));
        mock.install_driver = Arc::new(Mutex::new(vec![false; 5]));

        mock.download_verify_install(version_count)
            .await
            .unwrap_err();
    }
}
