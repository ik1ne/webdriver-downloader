use webdriver_downloader::{
    ChromedriverInfo, GeckodriverInfo, WebdriverDownloadError, WebdriverDownloadInfo,
};

use super::build_arg::*;
use super::check_arg::*;
use super::*;

pub async fn run() -> anyhow::Result<String> {
    let args = get_args()?;

    check_driver_install_path(&args.driver_install_path, args.mkdir)?;
    check_browser(&args.browser_path)?;

    match args.driver_type {
        DriverType::Chrome => {
            let driver_info = ChromedriverInfo::new(args.driver_install_path, args.browser_path);

            install_if_needed(&driver_info, 5)
                .await
                .map_err(|e| e.into())
        }
        DriverType::Gecko => {
            let driver_info = GeckodriverInfo::new(args.driver_install_path, args.browser_path);

            install_if_needed(&driver_info, 5)
                .await
                .map_err(|e| e.into())
        }
    }
}

async fn install_if_needed(
    driver_info: &impl WebdriverDownloadInfo,
    max_tries: usize,
) -> Result<String, WebdriverDownloadError> {
    if driver_info.is_installed().await {
        Ok("Driver already installed.".to_string())
    } else {
        driver_info.download_verify_install(max_tries).await?;
        Ok("Driver installed successfully.".to_string())
    }
}
