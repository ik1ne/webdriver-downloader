pub const ZIPFILE_NAME_RE: &str = r#"<Key>([0-9.]*?)/chromedriver_linux64.zip</Key>"#;
pub const DRIVER_NAME_IN_ARCHIVE: &str = "chromedriver";

pub fn build_url(version_string: &str) -> String {
    format!(
        "https://chromedriver.storage.googleapis.com/{}/chromedriver_linux64.zip",
        version_string
    )
}
