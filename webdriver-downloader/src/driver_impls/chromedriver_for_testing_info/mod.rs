use std::path::PathBuf;

use crate::driver_impls::chromedriver_info::OfflineVerificationError;
use crate::os_specific;
use crate::os_specific::DefaultPathError;
use crate::prelude::VersionReqUrlInfo;

mod trait_impls;

/// Information required to implement [WebdriverDownloadInfo](crate::prelude::WebdriverDownloadInfo) for Chrome for Testing.
pub struct ChromedriverForTestingInfo {
    pub driver_install_path: PathBuf,
    pub browser_path: PathBuf,
}

impl ChromedriverForTestingInfo {
    pub fn new(driver_install_path: PathBuf, browser_path: PathBuf) -> Self {
        ChromedriverForTestingInfo {
            driver_install_path,
            browser_path,
        }
    }

    /// Initialize ChromedriverForTestingInfo with default paths.
    ///
    /// # Errors
    ///
    /// Returns [`DefaultPathError`] if the default paths cannot be determined.
    pub fn new_default() -> Result<Self, DefaultPathError> {
        let driver_install_path = os_specific::chromedriver::default_driver_path()?;
        let browser_path = os_specific::chromedriver_for_testing::default_browser_path()?;

        Ok(ChromedriverForTestingInfo::new(
            driver_install_path,
            browser_path,
        ))
    }

    /// Verify that the driver and browser versions match, without making any network requests.
    pub fn verify_driver_offline(&self) -> Result<(), OfflineVerificationError> {
        let driver_version = os_specific::chromedriver::binary_version(&self.driver_install_path)
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
