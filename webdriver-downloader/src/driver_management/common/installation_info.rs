use std::fs;
use std::fs::File;
use std::io::{self, Cursor};
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use bytes::Bytes;
use reqwest::IntoUrl;
use tar::Archive;
use tempfile::TempDir;
use zip::ZipArchive;

#[derive(thiserror::Error, Debug)]
pub enum InstallationError {
    #[error("Failed to download driver: {0}")]
    Download(#[from] reqwest::Error),
    #[error("Unknown archive format.")]
    UnknownArchiveFormat,
    #[error("Failed to extract driver zipfile: {0}")]
    ExtractZip(#[from] zip::result::ZipError),
    #[error("Failed to extract driver tarball: {0}")]
    ExtractTar(io::Error),
    #[error("Failed to write driver to disk: {0}")]
    Write(io::Error),
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
    async fn download_in_tempdir<U: 'static + IntoUrl + AsRef<str> + Send>(
        &self,
        url: U,
        dir: &TempDir,
    ) -> Result<PathBuf, InstallationError> {
        let archive_type =
            detect_archive_type(url.as_ref()).ok_or(InstallationError::UnknownArchiveFormat)?;

        let response = reqwest::get(url).await?;
        let content = Cursor::new(response.bytes().await?);

        let driver_name = self.driver_name_in_archive();
        let driver_path = dir.path().join(driver_name);

        match archive_type {
            ArchiveType::Zip => {
                extract_zip(content, driver_name, &driver_path)?;
            }
            ArchiveType::TarGz => {
                extract_tarball(content, driver_name, &driver_path).await?;
            }
        }

        #[cfg(unix)]
        add_execute_permission(&driver_path)?;

        Ok(driver_path)
    }

    fn install_driver<P: AsRef<Path> + 'static>(
        &self,
        temp_driver_path: &P,
    ) -> Result<(), InstallationError> {
        fs::rename(temp_driver_path, self.driver_install_path()).map_err(InstallationError::Write)
    }
}

enum ArchiveType {
    Zip,
    TarGz,
}

fn detect_archive_type(url: &str) -> Option<ArchiveType> {
    if url.ends_with(".tar.gz") {
        Some(ArchiveType::TarGz)
    } else if url.ends_with(".zip") {
        Some(ArchiveType::Zip)
    } else {
        None
    }
}

fn extract_zip(
    content: Cursor<Bytes>,
    driver_name: &str,
    driver_path: &Path,
) -> Result<u64, InstallationError> {
    let mut archive = ZipArchive::new(content)?;
    let mut driver_content = archive.by_name(driver_name)?;

    let mut driver_file = File::create(driver_path).map_err(InstallationError::Write)?;
    io::copy(&mut driver_content, &mut driver_file).map_err(InstallationError::Write)
}

async fn extract_tarball(
    content: Cursor<Bytes>,
    driver_name: &str,
    driver_path: &Path,
) -> Result<(), InstallationError> {
    let tar = flate2::bufread::GzDecoder::new(content);
    let mut archive = Archive::new(tar);

    for entry_result in archive.entries().map_err(InstallationError::ExtractTar)? {
        let mut entry = entry_result.map_err(InstallationError::ExtractTar)?;
        if entry
            .path()
            .map_err(InstallationError::ExtractTar)?
            .ends_with(driver_name)
        {
            let mut driver_file = File::create(driver_path).map_err(InstallationError::Write)?;
            io::copy(&mut entry, &mut driver_file).map_err(InstallationError::Write)?;
            break;
        }
    }

    Ok(())
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
