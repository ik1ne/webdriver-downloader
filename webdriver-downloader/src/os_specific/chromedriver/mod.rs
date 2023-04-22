#[cfg(target_family = "unix")]
pub use unix_family::*;

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

#[cfg(target_family = "unix")]
mod unix_family;

use std::path::PathBuf;

use crate::os_specific::DefaultPathError;

pub fn default_driver_path() -> Result<PathBuf, DefaultPathError> {
    let home_dir = home::home_dir().ok_or(DefaultPathError::HomeDir)?;
    Ok(home_dir.join("bin").join(DRIVER_EXECUTABLE_NAME))
}
