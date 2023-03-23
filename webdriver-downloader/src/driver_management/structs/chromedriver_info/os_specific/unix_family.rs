use regex::Regex;
use semver::Version;
use std::path::Path;

pub fn binary_version(browser_path: &Path) -> Option<Version> {
    let re = Regex::new(r"([0-9\.]+)").expect("Failed to parse regex.");
    let output = std::process::Command::new(browser_path)
        .arg(Path::new("--version"))
        .output()
        .ok()?;

    let chrome_version_string = String::from_utf8_lossy(&output.stdout);
    let version_string = re
        .captures_iter(&chrome_version_string)
        .next()?
        .get(1)?
        .as_str()
        .to_string();

    lenient_semver::parse(&version_string).ok()
}
