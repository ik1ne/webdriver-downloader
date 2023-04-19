#[cfg(target_arch = "aarch64")]
pub const ZIPFILE_NAME_RE: &str = r#"<Key>([0-9.]*?)/chromedriver_mac_arm64.zip</Key>"#;
#[cfg(target_arch = "x86_64")]
pub const ZIPFILE_NAME_RE: &str = r#"<Key>([0-9.]*?)/chromedriver_mac64.zip</Key>"#;

pub const DRIVER_NAME_IN_ARCHIVE: &str = "chromedriver";

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
