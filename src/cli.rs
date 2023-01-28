use anyhow::{bail, Result};
use std::path::PathBuf;

mod build_arg;
mod check_arg;
mod run;

pub use run::run;

#[derive(Debug, Copy, Clone)]
pub(self) enum DriverType {
    Chrome,
    Gecko,
}

impl DriverType {
    fn driver_executable_name(&self) -> &'static str {
        match self {
            DriverType::Chrome => "chromedriver.exe",
            DriverType::Gecko => "geckodriver.exe",
        }
    }

    fn default_driver_install_path(&self) -> Result<PathBuf> {
        match home::home_dir() {
            Some(home) => Ok(home.join("bin").join(self.driver_executable_name())),
            None => {
                bail!("Failed to determine default driver install path.")
            }
        }
    }

    fn default_browser_path(&self) -> PathBuf {
        match self {
            DriverType::Chrome => {
                PathBuf::from(r"C:\Program Files\Google\Chrome\Application\chrome.exe")
            }
            DriverType::Gecko => todo!("Geckodriver is not implemented yet."),
        }
    }
}

#[derive(Debug)]
pub(self) struct Args {
    pub driver_type: DriverType,
    pub driver_install_path: PathBuf,
    pub browser_path: PathBuf,
    pub mkdir: bool,
}
