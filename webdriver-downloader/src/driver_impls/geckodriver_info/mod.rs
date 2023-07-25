use std::path::PathBuf;

use crate::os_specific;
use crate::os_specific::DefaultPathError;

mod trait_impls;

/// Information required to implement [WebdriverDownloadInfo](crate::prelude::WebdriverDownloadInfo) for Geckodriver.
pub struct GeckodriverInfo {
    pub driver_install_path: PathBuf,
    pub browser_path: PathBuf,
}

impl GeckodriverInfo {
    pub fn new(driver_install_path: PathBuf, browser_path: PathBuf) -> Self {
        GeckodriverInfo {
            driver_install_path,
            browser_path,
        }
    }

    pub fn new_default() -> Result<Self, DefaultPathError> {
        let driver_install_path = os_specific::geckodriver::default_driver_path()?;
        let browser_path = os_specific::geckodriver::default_browser_path()?;
        Ok(GeckodriverInfo::new(driver_install_path, browser_path))
    }
}
