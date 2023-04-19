use std::path::Path;

use regex::Regex;
use semver::Version;

use crate::common::version_req_url_info::VersionReqError;

pub fn binary_version(browser_path: &Path) -> Result<Version, VersionReqError> {
    let re = Regex::new(r"([0-9\.]+)").expect("Failed to parse regex.");
    let output = std::process::Command::new(browser_path)
        .arg(Path::new("--version"))
        .output()?;

    let chrome_version_string = String::from_utf8_lossy(&output.stdout);
    let version_string = capture_regex_from_string(&re, &chrome_version_string).ok_or(
        VersionReqError::RegexError(chrome_version_string.to_string()),
    )?;

    lenient_semver::parse(&version_string).map_err(|e| e.owned().into())
}

fn capture_regex_from_string(regex: &Regex, string: &str) -> Option<String> {
    let capture = regex.captures_iter(string).next()?;
    let regex_match = capture.get(1)?;
    Some(regex_match.as_str().to_string())
}
