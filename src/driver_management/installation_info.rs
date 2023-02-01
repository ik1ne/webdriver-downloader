use std::fs::File;
use std::io::{self, Cursor};
use std::path::{Path, PathBuf};

use anyhow::Result;
use async_trait::async_trait;
use reqwest::IntoUrl;
use tempfile::TempDir;
use zip::ZipArchive;

/// Provides information for installing driver.
#[async_trait]
pub trait WebdriverInstallationInfo {
    /// Path to install driver.
    fn driver_install_path(&self) -> &Path;

    /// Driver executable name in archive file.
    fn driver_name_in_zip(&self) -> &'static str;

    /// Downloads url and extracts the driver inside tempdir.
    async fn download_in_tempdir<U: IntoUrl + Send>(
        &self,
        url: U,
        dir: &TempDir,
    ) -> Result<PathBuf> {
        let response = reqwest::get(url).await?;

        let content = Cursor::new(response.bytes().await?);

        let mut archive = ZipArchive::new(content)?;
        let mut driver_content = archive.by_name(self.driver_name_in_zip())?;

        let driver_path = dir.path().join(self.driver_name_in_zip());
        let mut driver_file = File::create(&driver_path)?;
        io::copy(&mut driver_content, &mut driver_file)?;

        Ok(driver_path)
    }
}
