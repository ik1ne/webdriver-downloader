use anyhow::Result;
use tempfile::tempdir;

use super::build_arg::*;
use super::check_arg::*;
use super::*;

use crate::driver_management::chromedriver_info::ChromedriverInfo;
use crate::driver_management::download_verify_install;

pub async fn run() -> Result<()> {
    let args = get_args()?;

    check_driver_install_path(&args.driver_install_path, args.mkdir)?;
    check_browser(&args.browser_path)?;

    match args.driver_type {
        DriverType::Chrome => {
            let driver_info =
                ChromedriverInfo::new(args.driver_install_path, args.browser_path, tempdir()?);

            download_verify_install(driver_info).await
        }
        DriverType::Gecko => todo!("Geckodriver is not implemented yet."),
    }
}
