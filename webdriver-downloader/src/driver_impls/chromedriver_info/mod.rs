use std::path::{Path, PathBuf};

use semver::Version;

use crate::driver_impls::chromedriver_for_testing_info::ChromedriverForTestingInfo;
use crate::driver_impls::chromedriver_old_info::ChromedriverOldInfo;
use crate::os_specific;
use crate::os_specific::DefaultPathError;
use crate::traits::installation_info::WebdriverInstallationInfo;
use crate::traits::version_req_url_info::VersionReqError;
use crate::traits::version_req_url_info::VersionReqUrlInfo;

mod trait_impls;

#[derive(thiserror::Error, Debug)]
pub enum OfflineVerificationError {
    #[error("Failed to get driver version")]
    DriverVersion(VersionReqError),
    #[error("Failed to get binary version")]
    BinaryVersion(VersionReqError),
    #[error("Driver and browser versions have different major version({driver} != {browser})")]
    VersionMismatch { driver: Version, browser: Version },
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Information required to implement [WebdriverDownloadInfo](crate::prelude::WebdriverDownloadInfo) for Chromedriver.
/// This works on both old and new Chromedriver.
#[derive(Debug)]
pub enum ChromedriverInfo {
    OldInfo(ChromedriverOldInfo),
    NewInfo(ChromedriverForTestingInfo),
}

impl ChromedriverInfo {
    #[tracing::instrument]
    pub fn is_chrome_for_testing(path: &Path) -> Result<bool, VersionReqError> {
        const CHROME_FOR_TESTING_FIRST_MAJOR_VERSION: u64 = 116;

        os_specific::chromedriver::binary_version(path)
            .map(|version| version.major >= CHROME_FOR_TESTING_FIRST_MAJOR_VERSION)
    }

    #[tracing::instrument]
    pub fn new(driver_install_path: PathBuf, browser_path: PathBuf) -> Self {
        if Self::is_chrome_for_testing(&browser_path).unwrap_or(false) {
            ChromedriverInfo::NewInfo(ChromedriverForTestingInfo::new(
                driver_install_path,
                browser_path,
            ))
        } else {
            ChromedriverInfo::OldInfo(ChromedriverOldInfo::new(driver_install_path, browser_path))
        }
    }

    /// Initialize ChromedriverInfo with default paths.
    /// Searches for the new Chromedriver(Chrome for Testing) first, then the old Chromedriver.
    ///
    /// # Errors
    ///
    /// Returns [`DefaultPathError`] if the default paths cannot be determined.
    pub fn new_default() -> Result<Self, DefaultPathError> {
        let browser_path = os_specific::chromedriver_for_testing::default_browser_path()?;
        if browser_path.exists() {
            return Ok(ChromedriverInfo::NewInfo(
                ChromedriverForTestingInfo::new_default()?,
            ));
        }

        Ok(ChromedriverInfo::OldInfo(
            ChromedriverOldInfo::new_default()?
        ))
    }

    /// Set the path to install the driver to.
    pub fn set_install_path(&mut self, path: PathBuf) {
        match self {
            ChromedriverInfo::OldInfo(old_info) => old_info.driver_install_path = path,
            ChromedriverInfo::NewInfo(new_info) => new_info.driver_install_path = path,
        }
    }

    /// Set the path to the browser binary.
    pub fn set_browser_path(&mut self, path: PathBuf) {
        match self {
            ChromedriverInfo::OldInfo(old_info) => old_info.browser_path = path,
            ChromedriverInfo::NewInfo(new_info) => new_info.browser_path = path,
        }
    }

    /// Verify that the driver and browser versions match, without making any network requests.
    pub fn verify_driver_offline(&self) -> Result<(), OfflineVerificationError> {
        let driver_install_path = match self {
            ChromedriverInfo::OldInfo(old_info) => old_info.driver_install_path(),
            ChromedriverInfo::NewInfo(new_info) => new_info.driver_install_path(),
        };

        let driver_version = os_specific::chromedriver::binary_version(driver_install_path)
            .map_err(OfflineVerificationError::DriverVersion)?;
        let binary_version = self
            .binary_version()
            .map_err(OfflineVerificationError::BinaryVersion)?;

        if driver_version.major != binary_version.major {
            Err(OfflineVerificationError::VersionMismatch {
                driver: driver_version,
                browser: binary_version,
            })
        } else {
            Ok(())
        }
    }
}
