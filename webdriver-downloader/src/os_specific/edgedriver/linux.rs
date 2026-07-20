use std::path::PathBuf;
use which::which;

use crate::os_specific::DefaultPathError;

pub const DRIVER_EXECUTABLE_NAME: &str = "msedgedriver";

pub fn default_browser_path() -> Result<PathBuf, DefaultPathError> {
    // Try multiple possible Edge installation paths on Linux
    let possible_paths = [
        "microsoft-edge",
        "microsoft-edge-stable",
        "microsoft-edge-beta",
        "microsoft-edge-dev",
        "/opt/microsoft/msedge/microsoft-edge",
        "/usr/bin/microsoft-edge",
        "/usr/bin/microsoft-edge-stable",
    ];

    for path in &possible_paths {
        if let Ok(found_path) = which(path) {
            return Ok(found_path);
        }
    }

    // If no standard paths work, try using which with the most common name
    which("microsoft-edge").map_err(|e| e.into())
}
