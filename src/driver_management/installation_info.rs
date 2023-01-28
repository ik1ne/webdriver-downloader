use anyhow::Result;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// Provides information for installing driver.
pub trait WebdriverInstallationInfo {
    /// Path to install driver.
    fn driver_install_path(&self) -> &Path;

    /// Path to temp directory.
    fn temp_dir_path(&self) -> &Path;

    /// Driver executable name in archive file.
    fn driver_name_in_zip(&self) -> &'static str;
}

pub async fn download_in_tempdir<O: AsRef<OsStr>>(
    url: O,
    driver_info: &impl WebdriverInstallationInfo,
) -> Result<PathBuf> {
    todo!()
}
