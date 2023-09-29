use std::path::PathBuf;

use which::{which, Error};

use crate::os_specific::DefaultPathError;

pub const PLATFORM: &str = "linux64";

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
