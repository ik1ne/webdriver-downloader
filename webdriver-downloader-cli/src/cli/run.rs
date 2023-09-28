use webdriver_downloader::driver_impls::chromedriver_info::ChromedriverInfo;

use super::build_arg::*;
use super::check_arg::*;
use super::*;

pub async fn run() -> anyhow::Result<String> {
    let args = get_args()?;

    check_driver_install_path(&args)?;
    check_browser(&args)?;
    check_tries(&args)?;

    let install_result = match args.driver_type {
        DriverType::Chrome => {
            let driver_info = ChromedriverInfo::new(args.driver_install_path, args.browser_path);

            install(
                &driver_info,
                args.reinstall,
                args.skip_verification,
                args.num_tries,
            )
            .await
        }
        DriverType::Gecko => {
            let driver_info = GeckodriverInfo::new(args.driver_install_path, args.browser_path);

            install(
                &driver_info,
                args.reinstall,
                args.skip_verification,
                args.num_tries,
            )
            .await
        }
    };

    install_result.map_err(|e| e.into())
}

async fn install(
    driver_info: &impl WebdriverDownloadInfo,
    reinstall: bool,
    skip_verification: bool,
    num_tries: usize,
) -> Result<String, WebdriverDownloadError> {
    if !reinstall && driver_info.is_installed().await {
        Ok("Driver already installed.".to_string())
    } else {
        if skip_verification {
            driver_info.download_install().await?;
        } else {
            driver_info.download_verify_install(num_tries).await?;
        }

        Ok("Driver installed successfully.".to_string())
    }
}
