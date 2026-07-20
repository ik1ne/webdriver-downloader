use std::path::PathBuf;
use which::which;

use crate::os_specific::DefaultPathError;

pub const DRIVER_EXECUTABLE_NAME: &str = "geckodriver";

pub fn default_browser_path() -> Result<PathBuf, DefaultPathError> {
    which("firefox").map_err(|e| e.into())
}

pub fn build_url(version_string: &str) -> String {
    let platform = get_linux_platform();
    format!(
        "https://github.com/mozilla/geckodriver/releases/download/v{ver}/geckodriver-v{ver}-{platform}.tar.gz",
        ver=version_string,
        platform=platform
    )
}

#[cfg(target_arch = "x86_64")]
fn get_linux_platform() -> &'static str {
    "linux64"
}

#[cfg(target_arch = "aarch64")]
fn get_linux_platform() -> &'static str {
    "linux-aarch64"
}

// Fallback for other architectures
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
fn get_linux_platform() -> &'static str {
    "linux64" // Default to x64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_url_format() {
        let version = "0.33.0";
        let url = build_url(version);
        
        assert!(url.starts_with("https://github.com/mozilla/geckodriver/releases/download/"));
        assert!(url.contains(version));
        assert!(url.ends_with(".tar.gz"));
        
        // Check architecture-specific URL suffix
        #[cfg(target_arch = "x86_64")]
        {
            assert!(url.contains("linux64"));
            assert_eq!(url, "https://github.com/mozilla/geckodriver/releases/download/v0.33.0/geckodriver-v0.33.0-linux64.tar.gz");
        }
        
        #[cfg(target_arch = "aarch64")]
        {
            assert!(url.contains("linux-aarch64"));
            assert_eq!(url, "https://github.com/mozilla/geckodriver/releases/download/v0.33.0/geckodriver-v0.33.0-linux-aarch64.tar.gz");
        }
    }

    #[test]
    fn test_platform_detection() {
        let platform = get_linux_platform();
        
        #[cfg(target_arch = "x86_64")]
        assert_eq!(platform, "linux64");
        
        #[cfg(target_arch = "aarch64")]
        assert_eq!(platform, "linux-aarch64");
    }
}
