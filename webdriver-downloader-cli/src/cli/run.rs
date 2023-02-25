use anyhow::Result;

use webdriver_downloader::*;

use super::build_arg::*;
use super::check_arg::*;
use super::*;

pub async fn run() -> Result<()> {
    let args = get_args()?;

    check_driver_install_path(&args.driver_install_path, args.mkdir)?;
    check_browser(&args.browser_path)?;

    match args.driver_type {
        DriverType::Chrome => {
            let driver_info = ChromedriverInfo::new(args.driver_install_path, args.browser_path);

            download_verify_install(driver_info, 5).await
        }
        DriverType::Gecko => todo!("Geckodriver is not implemented yet."),
    }
}
