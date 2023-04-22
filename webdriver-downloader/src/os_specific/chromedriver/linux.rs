use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;

use crate::os_specific::DefaultPathError;

pub const ZIPFILE_NAME_RE: &str = r#"<Key>([0-9.]*?)/chromedriver_linux64.zip</Key>"#;
pub const DRIVER_EXECUTABLE_NAME: &str = "chromedriver";

pub fn default_browser_path() -> Result<PathBuf, DefaultPathError> {
    let child = std::process::Command::new("which")
        .arg("google-chrome")
        .output()?;

    Ok(PathBuf::from(OsString::from_vec(child.stdout)))
}

pub fn build_url(version_string: &str) -> String {
    format!(
        "https://chromedriver.storage.googleapis.com/{}/chromedriver_linux64.zip",
        version_string
    )
}
