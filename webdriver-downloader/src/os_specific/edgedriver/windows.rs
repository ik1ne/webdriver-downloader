use std::path::PathBuf;
use std::process::Command;

use crate::os_specific::DefaultPathError;
use crate::traits::version_req_url_info::VersionReqError;

pub const DRIVER_EXECUTABLE_NAME: &str = "msedgedriver.exe";

pub fn default_browser_path() -> Result<PathBuf, DefaultPathError> {
    // Check common Edge installation paths on Windows
    let program_files = std::env::var("ProgramFiles")?;
    let program_files_x86 = std::env::var("ProgramFiles(x86)")
        .unwrap_or_else(|_| "C:\\Program Files (x86)".to_string());

    let possible_paths = [
        // Stable channel paths
        format!("{program_files}\\Microsoft\\Edge\\Application\\msedge.exe"),
        format!("{program_files_x86}\\Microsoft\\Edge\\Application\\msedge.exe"),
        // Beta channel paths
        format!("{program_files}\\Microsoft\\Edge Beta\\Application\\msedge.exe"),
        format!("{program_files_x86}\\Microsoft\\Edge Beta\\Application\\msedge.exe"),
        // Dev channel paths
        format!("{program_files}\\Microsoft\\Edge Dev\\Application\\msedge.exe"),
        format!("{program_files_x86}\\Microsoft\\Edge Dev\\Application\\msedge.exe"),
        // Canary channel paths (per-user installation)
        format!(
            "{}\\Microsoft\\Edge SxS\\Application\\msedge.exe",
            std::env::var("LOCALAPPDATA")
                .unwrap_or_else(|_| "C:\\Users\\Default\\AppData\\Local".to_string())
        ),
    ];

    for path in &possible_paths {
        let path_buf = PathBuf::from(path);
        if path_buf.exists() {
            return Ok(path_buf);
        }
    }

    // Default to most common stable path if none exist (will be caught during verification)
    Ok(PathBuf::from(format!(
        "{program_files}\\Microsoft\\Edge\\Application\\msedge.exe"
    )))
}

/// Get Edge version on Windows using PowerShell to read file properties
/// This is needed because `msedge.exe --version` doesn't work properly on Windows
pub fn get_edge_version(browser_path: &std::path::Path) -> Result<String, VersionReqError> {
    // Use PowerShell to get the file version
    let powershell_script = format!(
        "(Get-Item '{}').VersionInfo.ProductVersion",
        browser_path.display().to_string().replace("\\", "\\\\")
    );

    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", &powershell_script])
        .output()
        .map_err(VersionReqError::Execute)?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        return Err(VersionReqError::RegexError(format!(
            "PowerShell command failed with exit code: {}",
            output.status.code().unwrap_or(-1)
        )));
    }

    let version_str = stdout.trim();
    if version_str.is_empty() {
        return Err(VersionReqError::RegexError(
            "PowerShell returned empty version string".to_string(),
        ));
    }

    Ok(version_str.to_string())
}
