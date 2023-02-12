use std::fs::rename;

use anyhow::{bail, Context};
use tempfile::TempDir;

pub use installation_info::WebdriverInstallationInfo;
pub use url_info::WebdriverUrlInfo;
pub use verification_info::WebdriverVerificationInfo;

pub mod chromedriver_info;
pub mod installation_info;
pub mod url_info;
pub mod verification_info;
mod binary_exact_version_hint_url_info;

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
) -> anyhow::Result<()> {
    let urls = driver_info.driver_urls(max_tries).await?;
    let url_count = urls.len();

    for url in urls {
        let tempdir = TempDir::new()?;

        let temp_driver_path = driver_info.download_in_tempdir(url, &tempdir).await?;

        if driver_info.verify_driver(&temp_driver_path).await.is_ok() {
            rename(temp_driver_path, driver_info.driver_install_path())
                .with_context(|| "Failed to install driver to driver_path.")?;

            return Ok(());
        }
    }

    bail!(
        "Tried {} possible versions, but no version passed verification.",
        url_count
    )
}
