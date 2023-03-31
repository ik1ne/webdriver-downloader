use std::fs;
use std::fs::File;
use std::io::{self, Cursor};
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use reqwest::IntoUrl;
use tempfile::TempDir;
use zip::ZipArchive;

#[derive(thiserror::Error, Debug)]
pub enum InstallationError {
    #[error("Failed to download driver: {0}")]
    Download(#[from] reqwest::Error),
    #[error("Failed to extract driver: {0}")]
    Extract(#[from] zip::result::ZipError),
    #[error("Failed to write driver to disk: {0}")]
    Write(#[from] io::Error),
    #[error(transparent)]
    AddExecutePermission(#[from] AddExecutePermissionError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Provides information for installing driver.
#[async_trait]
pub trait WebdriverInstallationInfo {
    /// Path to install driver.
    fn driver_install_path(&self) -> &Path;

    /// Driver executable name in archive file.
    fn driver_name_in_archive(&self) -> &'static str;

    /// Downloads url and extracts the driver inside tempdir.
    async fn download_in_tempdir<U: 'static + IntoUrl + Send>(
        &self,
        url: U,
        dir: &TempDir,
    ) -> Result<PathBuf, InstallationError> {
        let response = reqwest::get(url).await?;

        let content = Cursor::new(response.bytes().await?);

        let mut archive = ZipArchive::new(content)?;
        let mut driver_content = archive.by_name(self.driver_name_in_archive())?;

        let driver_path = dir.path().join(self.driver_name_in_archive());
        let mut driver_file = File::create(&driver_path)?;
        io::copy(&mut driver_content, &mut driver_file)?;

        #[cfg(unix)]
        add_execute_permission(&driver_path)?;

        Ok(driver_path)
    }

    fn install_driver<P: AsRef<Path> + 'static>(
        &self,
        temp_driver_path: &P,
    ) -> Result<(), InstallationError> {
        fs::rename(temp_driver_path, self.driver_install_path()).map_err(|e| e.into())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddExecutePermissionError {
    #[error("Failed to get file metadata: {0}")]
    Metadata(io::Error),
    #[error("Failed to set file permissions: {0}")]
    SetPermissions(io::Error),
}

#[cfg(unix)]
pub(crate) fn add_execute_permission(path: &Path) -> Result<(), AddExecutePermissionError> {
    use std::os::unix::fs::PermissionsExt;

    let metadata = path
        .metadata()
        .map_err(AddExecutePermissionError::Metadata)?;

    let mut permissions = metadata.permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).map_err(AddExecutePermissionError::SetPermissions)?;

    Ok(())
}
