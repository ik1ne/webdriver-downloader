use std::path::PathBuf;

use anyhow::{bail, Result};

pub use run::run;

mod build_arg;
mod check_arg;
mod run;

#[derive(Debug)]
pub(self) struct Args {
    pub driver_type: DriverType,
    pub driver_install_path: PathBuf,
    pub browser_path: PathBuf,
    pub mkdir: bool,
}

#[derive(Debug, Copy, Clone)]
pub(self) enum DriverType {
    Chrome,
    Gecko,
}



impl DriverType {
    fn default_driver_install_path(&self) -> Result<PathBuf> {
        match home::home_dir() {
            Some(home) => Ok(home.join("bin").join(self.driver_executable_name())),
            None => {
                bail!("Failed to determine default driver install path.")
            }
        }
    }
}

#[cfg(target_os = "windows")]
impl DriverType {
    fn driver_executable_name(&self) -> &'static str {
        match self {
            DriverType::Chrome => "chromedriver.exe",
            DriverType::Gecko => "geckodriver.exe",
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

#[cfg(target_os = "linux")]
impl DriverType {
    fn driver_executable_name(&self) -> &'static str {
        match self {
            DriverType::Chrome => "chromedriver",
            DriverType::Gecko => "geckodriver",
        }
    }

    fn default_browser_path(&self) -> PathBuf {
        match self {
            DriverType::Chrome => {
                PathBuf::from(r"/bin/google-chrome-stable")
            }
            DriverType::Gecko => todo!("Geckodriver is not implemented yet."),
        }
    }
}