use std::path::PathBuf;

use crate::os_specific::DefaultPathError;

pub const DRIVER_EXECUTABLE_NAME: &str = "msedgedriver";

pub fn default_browser_path() -> Result<PathBuf, DefaultPathError> {
    // Check for various Edge channels on macOS, in order of preference
    let possible_paths = [
        "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge",
        "/Applications/Microsoft Edge Beta.app/Contents/MacOS/Microsoft Edge Beta",
        "/Applications/Microsoft Edge Dev.app/Contents/MacOS/Microsoft Edge Dev",
        "/Applications/Microsoft Edge Canary.app/Contents/MacOS/Microsoft Edge Canary",
    ];

    for path in &possible_paths {
        let path_buf = PathBuf::from(path);
        if path_buf.exists() {
            return Ok(path_buf);
        }
    }

    // Default to stable channel path if none exist (will be caught during verification)
    Ok(PathBuf::from(
        "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge",
    ))
}
