use webdriver_downloader::driver_impls::{ChromedriverInfo, GeckodriverInfo};
use webdriver_downloader::WebdriverInfo;

use super::build_arg::*;
use super::check_arg::*;
use super::*;

pub async fn run() -> anyhow::Result<()> {
    let args = get_args()?;

    check_driver_install_path(&args.driver_install_path, args.mkdir)?;
    check_browser(&args.browser_path)?;

    match args.driver_type {
        DriverType::Chrome => {
            let driver_info = ChromedriverInfo::new(args.driver_install_path, args.browser_path);

            driver_info.download_verify_install(5).await?;
            Ok(())
        }
        DriverType::Gecko => {
            let driver_info = GeckodriverInfo::new(args.driver_install_path, args.browser_path);

            driver_info.download_verify_install(5).await?;
            Ok(())
        }
    }
}
