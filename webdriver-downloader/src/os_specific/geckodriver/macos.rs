use std::path::PathBuf;

use crate::os_specific::DefaultPathError;

pub const DRIVER_EXECUTABLE_NAME: &str = "geckodriver";

pub fn default_browser_path() -> Result<PathBuf, DefaultPathError> {
    Ok(PathBuf::from(
        r"/Applications/Firefox.app/Contents/MacOS/firefox",
    ))
}

#[cfg(target_arch = "aarch64")]
pub fn build_url(version_string: &str) -> String {
    format!(
        "https://github.com/mozilla/geckodriver/releases/download/v{ver}/geckodriver-v{ver}-macos-aarch64.tar.gz",
        ver=version_string
    )
}

#[cfg(target_arch = "x86_64")]
pub fn build_url(version_string: &str) -> String {
    format!(
        "https://github.com/mozilla/geckodriver/releases/download/v{ver}/geckodriver-v{ver}-macos.tar.gz",
        ver=version_string
    )
}
