pub const ZIPFILE_NAME_RE: &'static str = r"<Key>(.*?)/chromedriver_mac64_m1.zip</Key>";
pub const DRIVER_NAME_IN_ARCHIVE: &'static str = "chromedriver";

pub fn build_url(version_string: &str) -> String {
    format!(
        "https://chromedriver.storage.googleapis.com/{}/chromedriver_mac64_m1.zip",
        version_string
    )
}