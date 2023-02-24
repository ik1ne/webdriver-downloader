use std::fs::rename;

use anyhow::{bail, Context, Result};
use tempfile::TempDir;

pub use structs::ChromedriverInfo;
pub use traits::{WebdriverInstallationInfo, WebdriverUrlInfo, WebdriverVerificationInfo};

mod structs;
mod traits;

/// Information required to download, verify, install driver.
pub trait WebdriverInfo:
    WebdriverUrlInfo + WebdriverInstallationInfo + WebdriverVerificationInfo + Sync
{
}

impl<T> WebdriverInfo for T where
    T: WebdriverUrlInfo + WebdriverInstallationInfo + WebdriverVerificationInfo + Sync
{
}

pub async fn download_verify_install(
    driver_info: impl WebdriverInfo,
    max_tries: usize,
) -> Result<()> {
    let urls = driver_info.driver_urls(max_tries).await?;
    let url_count = urls.len();

    for url in urls {
        let tempdir = TempDir::new()?;

        let temp_driver_path = driver_info.download_in_tempdir(url, &tempdir).await?;

        if driver_info.verify_driver(&temp_driver_path).await.is_ok() {
            return rename(temp_driver_path, driver_info.driver_install_path())
                .with_context(|| "Failed to install driver to driver_path.");
        }
    }

    bail!(
        "Tried {} possible versions, but no version passed verification.",
        url_count
    )
}
