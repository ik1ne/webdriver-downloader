pub mod chromedriver_info;
pub mod installation_info;
pub mod url_info;
pub mod verification_info;

use anyhow::bail;
use std::fs::rename;
use std::path::PathBuf;

pub use installation_info::WebdriverInstallationInfo;
pub use url_info::WebdriverUrlInfo;
pub use verification_info::WebdriverVerificationInfo;

use crate::driver_management::installation_info::download_in_tempdir;
use crate::driver_management::verification_info::verify_driver;
use crate::WebdriverInfo;

pub async fn download_verify_install(driver_info: impl WebdriverInfo) -> anyhow::Result<()> {
    let urls = driver_info.driver_urls(5).await?;

    for url in urls {
        // as installation info
        let temp_driver_path: PathBuf = download_in_tempdir(url, &driver_info).await?;

        // as verification info
        if verify_driver(&temp_driver_path, &driver_info).await.is_ok() {
            rename(temp_driver_path, driver_info.driver_install_path())?;

            return Ok(());
        }
    }

    bail!("Tried all possible versions, but no version passed verification.")
}
