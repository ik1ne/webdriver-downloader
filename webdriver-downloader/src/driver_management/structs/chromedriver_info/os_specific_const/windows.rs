pub const ZIPFILE_NAME_RE: &'static str = r"<Key>(.*?)/chromedriver_win32.zip</Key>";
pub const DRIVER_NAME_IN_ARCHIVE: &'static str = "chromedriver.exe";

pub fn build_url(version_string: &str) -> String {
    format!(
        "https://chromedriver.storage.googleapis.com/{}/chromedriver_win32.zip",
        version_string
    )
}
