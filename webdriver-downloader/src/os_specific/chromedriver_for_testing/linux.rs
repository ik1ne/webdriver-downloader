use std::path::PathBuf;

use which::{which, Error};

use crate::os_specific::DefaultPathError;

#[cfg(target_arch = "x86_64")]
pub const PLATFORM: &str = "linux64";

// Chrome for Testing does not support ARM64 on Linux yet
// ARM64 users should use EdgeDriver instead
#[cfg(target_arch = "aarch64")]
pub const PLATFORM: &str = "linux64"; // Will result in 404, but allows compilation

pub const DRIVER_EXECUTABLE_NAME: &str = "chromedriver";

pub const BROWSER_EXECUTABLE_NAMES: &[&str] =
    &["google-chrome", "chrome", "chromium", "chromium-browser"];

pub fn default_browser_path() -> Result<PathBuf, DefaultPathError> {
    for name in BROWSER_EXECUTABLE_NAMES.iter() {
        match which(name) {
            Ok(path) => {
                return Ok(path);
            }
            Err(e) => match e {
                Error::CannotFindBinaryPath => continue,
                _ => return Err(DefaultPathError::Which(e)),
            },
        }
    }

    Err(DefaultPathError::BinaryNotFound)
}
