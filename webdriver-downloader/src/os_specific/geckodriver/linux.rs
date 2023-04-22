use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;

use crate::os_specific::DefaultPathError;

pub const DRIVER_EXECUTABLE_NAME: &str = "geckodriver";

pub fn default_browser_path() -> Result<PathBuf, DefaultPathError> {
    let child = std::process::Command::new("which")
        .arg("firefox")
        .output()?;

    Ok(PathBuf::from(OsString::from_vec(child.stdout)))
}

pub fn build_url(version_string: &str) -> String {
    format!(
        "https://github.com/mozilla/geckodriver/releases/download/v{ver}/geckodriver-v{ver}-linux64.tar.gz",
        ver=version_string
    )
}
