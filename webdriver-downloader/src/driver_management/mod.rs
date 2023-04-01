use std::io;

use async_trait::async_trait;
use tempfile::TempDir;

use crate::common::installation_info::{InstallationError, WebdriverInstallationInfo};
use crate::common::url_info::{UrlError, WebdriverUrlInfo};
use crate::common::verification_info::{VerificationError, WebdriverVerificationInfo};

pub mod common;
pub mod driver_impls;

/// Information required to download, verify, install driver.
#[async_trait]
pub trait WebdriverInfo:
    WebdriverUrlInfo + WebdriverInstallationInfo + WebdriverVerificationInfo + Sync
{
    async fn download_verify_install(&self, max_tries: usize)
        -> Result<(), WebdriverDownloadError>;
}

#[derive(thiserror::Error, Debug)]
pub enum WebdriverDownloadError {
    #[error(transparent)]
    Url(#[from] UrlError),
    #[error(transparent)]
    Install(#[from] InstallationError),
    #[error(transparent)]
    Verify(#[from] VerificationError),
    #[error("Failed to move driver to driver_path: {0}")]
    Move(#[from] io::Error),
    #[error("Tried {0} possible versions, but no version passed verification.")]
    NoVersionPassedVerification(usize),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[async_trait]
impl<T> WebdriverInfo for T
where
    T: WebdriverUrlInfo + WebdriverInstallationInfo + WebdriverVerificationInfo + Sync,
{
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
