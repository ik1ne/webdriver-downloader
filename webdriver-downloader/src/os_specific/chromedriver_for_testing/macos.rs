use std::path::PathBuf;

use crate::os_specific::DefaultPathError;

#[cfg(target_arch = "aarch64")]
pub const PLATFORM: &str = "mac-arm64";
#[cfg(target_arch = "x86_64")]
pub const PLATFORM: &str = "mac-x64";

pub const DRIVER_EXECUTABLE_NAME: &str = "chromedriver";

pub fn default_browser_path() -> Result<PathBuf, DefaultPathError> {
    Ok(PathBuf::from(
        r"/Applications/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing",
    ))
}
