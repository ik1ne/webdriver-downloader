use std::path::Path;
use std::process::Stdio;

use regex::Regex;
use semver::Version;
use tracing::trace;

use crate::traits::version_req_url_info::VersionReqError;

#[tracing::instrument]
pub fn binary_version(browser_path: &Path) -> Result<Version, VersionReqError> {
    let re = Regex::new(r"([0-9\.]+)").expect("Failed to parse regex.");
    let output = std::process::Command::new(browser_path)
        .arg(Path::new("--version"))
        .stderr(Stdio::piped())
        .output()?;

    let gecko_version_string = String::from_utf8_lossy(&output.stdout);
    trace!("Gecko version string: {}", gecko_version_string);
    let version_string = capture_regex_from_string(&re, &gecko_version_string).ok_or(
        VersionReqError::RegexError(gecko_version_string.to_string()),
    )?;

    lenient_semver::parse(&version_string).map_err(|e| e.owned().into())
}

fn capture_regex_from_string(regex: &Regex, string: &str) -> Option<String> {
    let capture = regex.captures_iter(string).next()?;
    let regex_match = capture.get(1)?;
    Some(regex_match.as_str().to_string())
}
