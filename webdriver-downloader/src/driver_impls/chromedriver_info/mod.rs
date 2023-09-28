use std::path::{Path, PathBuf};

use semver::Version;

use crate::os_specific;
use crate::os_specific::DefaultPathError;
use crate::traits::version_req_url_info::VersionReqError;

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
pub struct ChromedriverInfo {
    old_info: Option<ChromedriverOldInfo>,
    new_info: Option<ChromedriverForTestingInfo>,
}

impl ChromedriverInfo {
    pub fn is_chrome_for_testing(path: &Path) -> bool {
    }
    
    pub fn new(driver_install_path: PathBuf, browser_path: PathBuf) -> Self {
        
    }

    /// Initialize ChromedriverInfo with default paths.
    /// Searches for the new Chromedriver(Chrome for Testing) first, then the old Chromedriver.
    ///
    /// # Errors
    ///
    /// Returns [`DefaultPathError`] if the default paths cannot be determined.
    pub fn new_default() -> Result<Self, DefaultPathError> {
        let driver_install_path = os_specific::chromedriver_for_testing::default_driver_path()?;
        let browser_path = os_specific::chromedriver_for_testing::default_browser_path()?;

        if browser_path.exists() {
            Ok(ChromedriverInfo::new(driver_install_path, browser_path))
        } else {
            let driver_install_path = os_specific::chromedriver_old::default_driver_path()?;
            let browser_path = os_specific::chromedriver_old::default_browser_path()?;
            Ok(ChromedriverInfo::new(driver_install_path, browser_path))
        }
    }

    /// Verify that the driver and browser versions match, without making any network requests.
    pub fn verify_driver_offline(&self) -> Result<(), OfflineVerificationError> {
        let driver_version =
            os_specific::chromedriver_old::binary_version(&self.driver_install_path)
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
