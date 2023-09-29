use std::path::PathBuf;

use crate::os_specific::DefaultPathError;

#[cfg(target_arch = "x86")]
pub const PLATFORM: &str = "win32";

#[cfg(target_arch = "x86_64")]
pub const PLATFORM: &str = "win64";

pub const DRIVER_EXECUTABLE_NAME: &str = "chromedriver";

pub fn default_browser_path() -> Result<PathBuf, DefaultPathError> {
    let program_files = std::env::var("ProgramFiles")?;
    Ok(PathBuf::from(format!(
        r"{}\Google\Chrome\Application\chrome.exe",
        program_files
    )))
}
