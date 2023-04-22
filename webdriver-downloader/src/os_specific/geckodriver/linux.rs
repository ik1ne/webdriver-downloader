use which::which;
use std::path::PathBuf;

use crate::os_specific::DefaultPathError;

pub const DRIVER_EXECUTABLE_NAME: &str = "geckodriver";

pub fn default_browser_path() -> Result<PathBuf, DefaultPathError> {
    which("firefox").map_err(|e| e.into())
}

pub fn build_url(version_string: &str) -> String {
    format!(
        "https://github.com/mozilla/geckodriver/releases/download/v{ver}/geckodriver-v{ver}-linux64.tar.gz",
        ver=version_string
    )
}
