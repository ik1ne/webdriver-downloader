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
        let program_files = std::env::var("ProgramFiles").unwrap();
        match self {
            DriverType::Chrome => PathBuf::from(format!(
                r"{}\Google\Chrome\Application\chrome.exe",
                program_files
            )),
            DriverType::Gecko => {
                PathBuf::from(format!(r"{}\Mozilla Firefox\firefox.exe", program_files))
            }
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
            DriverType::Chrome => PathBuf::from(r"/bin/google-chrome-stable"),
            DriverType::Gecko => PathBuf::from(r"/bin/firefox"),
        }
    }
}

#[cfg(target_os = "macos")]
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
                PathBuf::from(r"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome")
            }
            DriverType::Gecko => PathBuf::from(r"/Applications/Firefox.app/Contents/MacOS/firefox"),
        }
    }
}
