use std::path::PathBuf;

use crate::os_specific::DefaultPathError;

#[cfg(target_arch = "aarch64")]
pub const ZIPFILE_NAME_RE: &str = r#"<Key>([0-9.]*?)/chromedriver_mac_arm64.zip</Key>"#;
#[cfg(target_arch = "x86_64")]
pub const ZIPFILE_NAME_RE: &str = r#"<Key>([0-9.]*?)/chromedriver_mac64.zip</Key>"#;

pub fn default_browser_path() -> Result<PathBuf, DefaultPathError> {
    Ok(PathBuf::from(
        r"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
    ))
}

#[cfg(target_arch = "aarch64")]
pub fn build_url(version_string: &str) -> String {
    format!(
        "https://chromedriver.storage.googleapis.com/{}/chromedriver_mac_arm64.zip",
        version_string
    )
}

#[cfg(target_arch = "x86_64")]
pub fn build_url(version_string: &str) -> String {
    format!(
        "https://chromedriver.storage.googleapis.com/{}/chromedriver_mac64.zip",
        version_string
    )
}
