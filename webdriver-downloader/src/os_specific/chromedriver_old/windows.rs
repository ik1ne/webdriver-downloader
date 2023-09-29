use std::path::PathBuf;

use crate::os_specific::DefaultPathError;

pub const ZIPFILE_NAME_RE: &str = r#"<Key>([0-9.]*?)/chromedriver_win32.zip</Key>"#;

pub fn default_browser_path() -> Result<PathBuf, DefaultPathError> {
    let program_files = std::env::var("ProgramFiles")?;
    Ok(PathBuf::from(format!(
        r"{}\Google\Chrome\Application\chrome.exe",
        program_files
    )))
}

pub fn build_url(version_string: &str) -> String {
    format!(
        "https://chromedriver.storage.googleapis.com/{}/chromedriver_win32.zip",
        version_string
    )
}
