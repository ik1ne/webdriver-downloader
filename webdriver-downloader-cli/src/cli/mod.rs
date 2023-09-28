use std::path::PathBuf;

pub use run::run;
use webdriver_downloader::prelude::*;

mod build_arg;
mod check_arg;
mod run;

#[derive(Debug)]
pub(crate) struct Args {
    pub driver_type: DriverType,
    pub driver_install_path: PathBuf,
    pub browser_path: PathBuf,
    pub num_tries: usize,
    pub skip_verification: bool,
    pub mkdir: bool,
    pub reinstall: bool,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum DriverType {
    Chrome,
    Gecko,
}

impl DriverType {
    fn default_driver_install_path(&self) -> Result<PathBuf, os_specific::DefaultPathError> {
        match self {
            DriverType::Chrome => os_specific::chromedriver::default_driver_path(),
            DriverType::Gecko => os_specific::geckodriver::default_driver_path(),
        }
    }

    fn default_browser_path(&self) -> Result<PathBuf, os_specific::DefaultPathError> {
        match self {
            DriverType::Chrome => {
                let path_chrome_for_testing =
                    os_specific::chromedriver_for_testing::default_browser_path()?;
                if path_chrome_for_testing.exists() {
                    Ok(path_chrome_for_testing)
                } else {
                    os_specific::chromedriver_old::default_browser_path()
                }
            }
            DriverType::Gecko => os_specific::geckodriver::default_browser_path(),
        }
    }
}
