use std::path::PathBuf;

use crate::os_specific;
use crate::os_specific::DefaultPathError;

mod trait_impls;

/// Information required to implement [WebdriverDownloadInfo](crate::prelude::WebdriverDownloadInfo) for Microsoft EdgeDriver.
#[derive(Debug)]
pub struct EdgedriverInfo {
    pub driver_install_path: PathBuf,
    pub browser_path: PathBuf,
}

impl EdgedriverInfo {
    #[tracing::instrument]
    pub fn new(driver_install_path: PathBuf, browser_path: PathBuf) -> Self {
        EdgedriverInfo {
            driver_install_path,
            browser_path,
        }
    }

    pub fn new_default() -> Result<Self, DefaultPathError> {
        let driver_install_path = os_specific::edgedriver::default_driver_path()?;
        let browser_path = os_specific::edgedriver::default_browser_path()?;
        Ok(EdgedriverInfo::new(driver_install_path, browser_path))
    }
}
