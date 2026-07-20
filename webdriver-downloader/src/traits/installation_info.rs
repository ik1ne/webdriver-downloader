use std::ffi::OsStr;
use std::fmt::Debug;
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

/// Error that can occur during installation.
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
    /// Path to install driver to.
    fn driver_install_path(&self) -> &Path;

    /// Driver executable name.
    fn driver_executable_name(&self) -> &str;

    /// Downloads url and extracts the driver executable to tempdir.
    #[tracing::instrument(skip(self))]
    async fn download_in_tempdir<U: IntoUrl + AsRef<str> + Debug + Send>(
        &self,
        url: U,
        dir: &TempDir,
    ) -> Result<PathBuf, InstallationError> {
        let archive_type =
            detect_archive_type(url.as_ref()).ok_or(InstallationError::UnknownArchiveFormat)?;

        tracing::debug!("Starting download from: {}", url.as_ref());
        
        // Create a client with proper timeout settings
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(300)) // 5 minutes timeout
            .connect_timeout(std::time::Duration::from_secs(30)) // 30 seconds connect timeout
            .build()
            .map_err(|e| InstallationError::Download(e))?;
            
        let response = client.get(url).send().await?;
        
        // Check if response is successful
        if !response.status().is_success() {
            return Err(InstallationError::Other(anyhow::anyhow!(
                "Download failed with HTTP status: {}", 
                response.status()
            )));
        }
        
        tracing::debug!("Download response status: {}", response.status());
        let content_length = response.content_length();
        if let Some(len) = content_length {
            tracing::debug!("Expected content length: {} bytes", len);
        }
        
        let bytes = response.bytes().await?;
        tracing::debug!("Downloaded {} bytes", bytes.len());
        let content = Cursor::new(bytes);

        let driver_executable_name = self.driver_executable_name();
        let driver_path = dir.path().join(driver_executable_name);

        match archive_type {
            ArchiveType::Zip => {
                extract_zip(content, driver_executable_name, &driver_path)?;
            }
            ArchiveType::TarGz => {
                extract_tarball(content, driver_executable_name, &driver_path)?;
            }
        }

        #[cfg(unix)]
        add_execute_permission(&driver_path)?;

        Ok(driver_path)
    }

    /// installs driver from `temp_dir_path` to [`self.driver_install_path()`](Self::driver_install_path).
    #[tracing::instrument(skip(self))]
    fn install_driver<P: AsRef<Path> + Debug>(
        &self,
        temp_driver_path: &P,
    ) -> Result<(), InstallationError> {
        fs::rename(temp_driver_path, self.driver_install_path())
            .or_else(|e| {
                // io::ErrorKind::CrossesDevices => try to copy instead
                if let Some(18) = e.raw_os_error() {
                    fs::copy(temp_driver_path, self.driver_install_path())
                        .and_then(|_| fs::remove_file(temp_driver_path))
                } else {
                    Err(e)
                }
            })
            .map_err(InstallationError::Write)
    }
}

enum ArchiveType {
    Zip,
    TarGz,
}

#[tracing::instrument]
fn detect_archive_type(url: &str) -> Option<ArchiveType> {
    if url.ends_with(".tar.gz") {
        Some(ArchiveType::TarGz)
    } else if url.ends_with(".zip") {
        Some(ArchiveType::Zip)
    } else {
        None
    }
}

#[tracing::instrument(skip(content))]
fn extract_zip(
    content: Cursor<Bytes>,
    driver_executable_name: &str,
    driver_path: &Path,
) -> Result<u64, InstallationError> {
    let mut archive = ZipArchive::new(content)?;

    let file_names = archive.file_names().map(str::to_string).collect::<Vec<_>>();

    // file_names are actually file_paths
    for file_name in file_names {
        let file_path = Path::new(&file_name);
        let real_file_name = file_path.file_name();
        if real_file_name == Some(OsStr::new(driver_executable_name)) {
            let mut driver_file = File::create(driver_path).map_err(InstallationError::Write)?;
            let mut driver_content = archive.by_name(&file_name)?;
            return io::copy(&mut driver_content, &mut driver_file)
                .map_err(InstallationError::Write);
        }
    }
    let mut driver_content = archive.by_name(driver_executable_name)?;

    let mut driver_file = File::create(driver_path).map_err(InstallationError::Write)?;
    io::copy(&mut driver_content, &mut driver_file).map_err(InstallationError::Write)
}

#[tracing::instrument(skip(content))]
fn extract_tarball(
    content: Cursor<Bytes>,
    driver_executable_name: &str,
    driver_path: &Path,
) -> Result<(), InstallationError> {
    tracing::debug!("Starting TAR.GZ extraction for executable: {}", driver_executable_name);
    
    let tar = flate2::bufread::GzDecoder::new(content);
    let mut archive = Archive::new(tar);
    let mut found_executable = false;
    let mut entry_names = Vec::new();

    for entry_result in archive.entries().map_err(InstallationError::ExtractTar)? {
        let mut entry = entry_result.map_err(InstallationError::ExtractTar)?;
        let entry_path = entry.path().map_err(InstallationError::ExtractTar)?;
        let entry_file_name = entry_path.file_name();
        
        // Log the entry path for debugging
        entry_names.push(entry_path.to_string_lossy().to_string());
        tracing::debug!("Found tar entry: {:?}", entry_path);
        
        if entry_file_name == Some(OsStr::new(driver_executable_name)) {
            tracing::debug!("Found matching executable in tar: {:?}", entry_path);
            let mut driver_file = File::create(driver_path).map_err(InstallationError::Write)?;
            let bytes_copied = io::copy(&mut entry, &mut driver_file).map_err(InstallationError::Write)?;
            tracing::debug!("Extracted {} bytes to {:?}", bytes_copied, driver_path);
            
            // Validate that we actually extracted something meaningful
            if bytes_copied == 0 {
                return Err(InstallationError::Other(anyhow::anyhow!(
                    "Extracted executable '{}' is empty (0 bytes)",
                    driver_executable_name
                )));
            }
            
            // Verify the file exists and has content
            let metadata = driver_path.metadata().map_err(InstallationError::Write)?;
            if metadata.len() == 0 {
                return Err(InstallationError::Other(anyhow::anyhow!(
                    "Extracted executable '{}' file is empty after extraction",
                    driver_executable_name
                )));
            }
            
            tracing::debug!("Verified extracted file size: {} bytes", metadata.len());
            found_executable = true;
            break;
        }
    }
    
    if !found_executable {
        tracing::error!(
            "Failed to find executable '{}' in tar archive. Found entries: {:?}",
            driver_executable_name,
            entry_names
        );
        return Err(InstallationError::Other(anyhow::anyhow!(
            "Executable '{}' not found in tar archive. Available entries: {:?}",
            driver_executable_name,
            entry_names
        )));
    }

    tracing::debug!("TAR.GZ extraction completed successfully");
    Ok(())
}

/// Error that can occur during adding execute permission.
#[derive(thiserror::Error, Debug)]
pub enum AddExecutePermissionError {
    #[error("Failed to get file metadata: {0}")]
    Metadata(io::Error),
    #[error("Failed to set file permissions: {0}")]
    SetPermissions(io::Error),
}

#[cfg(unix)]
#[tracing::instrument]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use tar::Builder;
    use tempfile::tempdir;

    /// Create a test TAR.GZ archive containing a mock executable file
    fn create_test_tar_gz(executable_name: &str, content: &[u8]) -> Vec<u8> {
        let mut tar_data = Vec::new();
        {
            let mut tar = Builder::new(&mut tar_data);
            
            let mut header = tar::Header::new_gnu();
            header.set_path(executable_name).unwrap();
            header.set_size(content.len() as u64);
            header.set_mode(0o755);
            header.set_cksum();
            
            tar.append(&header, content).unwrap();
            tar.finish().unwrap();
        }
        
        // Compress with gzip
        let mut gz_data = Vec::new();
        {
            let mut encoder = GzEncoder::new(&mut gz_data, Compression::default());
            encoder.write_all(&tar_data).unwrap();
            encoder.finish().unwrap();
        }
        
        gz_data
    }

    #[test]
    fn test_extract_tarball_success() {
        let temp_dir = tempdir().unwrap();
        let executable_name = "test_driver";
        let test_content = b"This is a test executable content";
        
        // Create test TAR.GZ
        let tar_gz_data = create_test_tar_gz(executable_name, test_content);
        let cursor = Cursor::new(Bytes::from(tar_gz_data));
        
        // Extract
        let output_path = temp_dir.path().join(executable_name);
        let result = extract_tarball(cursor, executable_name, &output_path);
        
        // Verify
        assert!(result.is_ok(), "TAR.GZ extraction should succeed");
        assert!(output_path.exists(), "Extracted file should exist");
        
        let extracted_content = std::fs::read(&output_path).unwrap();
        assert_eq!(extracted_content, test_content, "Extracted content should match original");
    }

    #[test]
    fn test_extract_tarball_executable_not_found() {
        let temp_dir = tempdir().unwrap();
        let executable_name = "missing_driver";
        let test_content = b"This is a test executable content";
        
        // Create test TAR.GZ with different filename
        let tar_gz_data = create_test_tar_gz("different_name", test_content);
        let cursor = Cursor::new(Bytes::from(tar_gz_data));
        
        // Extract
        let output_path = temp_dir.path().join(executable_name);
        let result = extract_tarball(cursor, executable_name, &output_path);
        
        // Verify
        assert!(result.is_err(), "TAR.GZ extraction should fail when executable not found");
        assert!(!output_path.exists(), "Output file should not exist");
        
        if let Err(InstallationError::Other(err)) = result {
            let error_msg = err.to_string();
            assert!(error_msg.contains("not found in tar archive"), 
                "Error should mention executable not found: {}", error_msg);
        } else {
            panic!("Expected InstallationError::Other");
        }
    }

    #[test]
    fn test_detect_archive_type() {
        assert!(matches!(detect_archive_type("file.tar.gz"), Some(ArchiveType::TarGz)));
        assert!(matches!(detect_archive_type("file.zip"), Some(ArchiveType::Zip)));
        assert!(matches!(detect_archive_type("file.txt"), None));
        
        // Test with URLs
        assert!(matches!(
            detect_archive_type("https://example.com/driver.tar.gz"), 
            Some(ArchiveType::TarGz)
        ));
        assert!(matches!(
            detect_archive_type("https://example.com/driver.zip"), 
            Some(ArchiveType::Zip)
        ));
    }
}
