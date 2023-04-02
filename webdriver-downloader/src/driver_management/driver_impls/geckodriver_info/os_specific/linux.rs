pub const DRIVER_NAME_IN_ARCHIVE: &str = "geckodriver";

pub fn build_url(version_string: &str) -> String {
    format!(
        "https://github.com/mozilla/geckodriver/releases/download/v{ver}/geckodriver-v{ver}-linux64.tar.gz",
        ver=version_string
    )
}
