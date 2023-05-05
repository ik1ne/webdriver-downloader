use std::path::PathBuf;
use which::{which, Error};

use crate::os_specific::DefaultPathError;

pub const ZIPFILE_NAME_RE: &str = r#"<Key>([0-9.]*?)/chromedriver_linux64.zip</Key>"#;
pub const DRIVER_EXECUTABLE_NAME: &str = "chromedriver";

pub const BROWSER_EXECUTABLE_NAMES: &[&str] = &["google-chrome", "chrome", "chromium"];

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

pub fn build_url(version_string: &str) -> String {
    format!(
        "https://chromedriver.storage.googleapis.com/{}/chromedriver_linux64.zip",
        version_string
    )
}
