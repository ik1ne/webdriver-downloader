use std::path::PathBuf;

#[cfg(target_family = "unix")]
pub use unix_family::*;
#[cfg(target_os = "windows")]
pub use windows::*;

use crate::os_specific::DefaultPathError;

#[cfg(target_family = "unix")]
mod unix_family;
#[cfg(target_os = "windows")]
mod windows;

pub fn default_driver_path() -> Result<PathBuf, DefaultPathError> {
    let home_dir = home::home_dir().ok_or(DefaultPathError::HomeDir)?;
    Ok(home_dir.join("bin").join(DRIVER_EXECUTABLE_NAME))
}
