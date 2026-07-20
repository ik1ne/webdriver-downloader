#[cfg(target_os = "linux")]
pub use linux::*;
#[cfg(target_os = "macos")]
pub use macos::*;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use std::path::PathBuf;

#[cfg(not(target_os = "windows"))]
use std::process::Command;

use crate::os_specific::DefaultPathError;
use crate::traits::version_req_url_info::VersionReqError;

pub fn default_driver_path() -> Result<PathBuf, DefaultPathError> {
    let home_dir = home::home_dir().ok_or(DefaultPathError::HomeDir)?;
    Ok(home_dir.join("bin").join(DRIVER_EXECUTABLE_NAME))
}

pub fn binary_version(browser_path: &std::path::Path) -> Result<semver::Version, VersionReqError> {
    let version_str = get_version_string(browser_path)?;

    // Edge uses a 4-component version (major.minor.patch.build)
    // We need to convert this to a 3-component semver
    // Take the first 3 components as major.minor.patch
    let parts: Vec<&str> = version_str.split('.').collect();

    if parts.len() >= 3 {
        let semver_str = format!("{}.{}.{}", parts[0], parts[1], parts[2]);
        semver::Version::parse(&semver_str).map_err(|e| {
            VersionReqError::RegexError(format!("Failed to parse semver '{}': {}", semver_str, e))
        })
    } else {
        // Fallback to lenient parsing if format is unexpected
        lenient_semver::parse(&version_str).map_err(|e| VersionReqError::ParseVersion(e.owned()))
    }
}

pub fn binary_version_string(browser_path: &std::path::Path) -> Result<String, VersionReqError> {
    get_version_string(browser_path)
}

#[cfg(target_os = "windows")]
fn get_version_string(browser_path: &std::path::Path) -> Result<String, VersionReqError> {
    // On Windows, use PowerShell to get file version properties
    // because msedge.exe --version doesn't work properly
    crate::os_specific::edgedriver::get_edge_version(browser_path)
}

#[cfg(not(target_os = "windows"))]
fn get_version_string(browser_path: &std::path::Path) -> Result<String, VersionReqError> {
    // On Unix-like systems, use --version command
    let output = Command::new(browser_path)
        .arg("--version")
        .output()
        .map_err(VersionReqError::Execute)?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Microsoft Edge version format: "Microsoft Edge 141.0.3537.57"
    let version_str = stdout
        .split_whitespace()
        .last()
        .ok_or_else(|| {
            VersionReqError::RegexError(format!(
                "Unable to parse Edge version from output: '{}'",
                stdout.trim()
            ))
        })?
        .trim();

    Ok(version_str.to_string())
}

pub fn build_url(version_string: &str) -> String {
    format!(
        "https://msedgedriver.microsoft.com/{ver}/edgedriver_{platform}.zip",
        ver = version_string,
        platform = platform_string()
    )
}

#[cfg(all(target_os = "windows", target_arch = "x86"))]
fn platform_string() -> &'static str {
    "win32"
}

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
fn platform_string() -> &'static str {
    "win64"
}

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
fn platform_string() -> &'static str {
    "mac64"
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn platform_string() -> &'static str {
    "mac64_m1"
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn platform_string() -> &'static str {
    "linux64"
}

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
fn platform_string() -> &'static str {
    "arm64"
}

#[cfg(all(target_os = "windows", target_arch = "aarch64"))]
fn platform_string() -> &'static str {
    "arm64"
}

// Fallback for other Windows architectures
#[cfg(all(
    target_os = "windows",
    not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))
))]
fn platform_string() -> &'static str {
    "win64" // Default to win64 for unknown Windows architectures
}
