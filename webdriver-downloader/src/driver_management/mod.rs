use std::{fs, io};

use tempfile::TempDir;

use crate::traits::installation_info::{InstallationError, WebdriverInstallationInfo};
use crate::traits::url_info::{UrlError, WebdriverUrlInfo};
use crate::traits::verification_info::{VerificationError, WebdriverVerificationInfo};

pub mod structs;
pub mod traits;

/// Information required to download, verify, install driver.
pub trait WebdriverInfo:
    WebdriverUrlInfo + WebdriverInstallationInfo + WebdriverVerificationInfo + Sync
{
}

impl<T> WebdriverInfo for T where
    T: WebdriverUrlInfo + WebdriverInstallationInfo + WebdriverVerificationInfo + Sync
{
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
}

pub async fn download_verify_install(
    driver_info: impl WebdriverInfo,
    max_tries: usize,
) -> Result<(), WebdriverDownloadError> {
    let urls = driver_info.driver_urls(max_tries).await?;
    let url_count = urls.len();

    for url in urls {
        println!("Trying url: {:?}.", url);
        let tempdir = TempDir::new()?;

        let temp_driver_path = driver_info.download_in_tempdir(url, &tempdir).await?;

        match driver_info.verify_driver(&temp_driver_path).await {
            Ok(_) => {
                fs::rename(temp_driver_path, driver_info.driver_install_path())?;
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
