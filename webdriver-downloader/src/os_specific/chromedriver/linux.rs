use std::path::PathBuf;
use which::which;

use crate::os_specific::DefaultPathError;

pub const ZIPFILE_NAME_RE: &str = r#"<Key>([0-9.]*?)/chromedriver_linux64.zip</Key>"#;
pub const DRIVER_EXECUTABLE_NAME: &str = "chromedriver";

pub fn default_browser_path() -> Result<PathBuf, DefaultPathError> {
    which("google-chrome").map_err(|e| e.into())
}

pub fn build_url(version_string: &str) -> String {
    format!(
        "https://chromedriver.storage.googleapis.com/{}/chromedriver_linux64.zip",
        version_string
    )
}
