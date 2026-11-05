//! Pins the API that downstream crates rely on.

use std::error::Error;

use webdriver_downloader::driver_impls::chromedriver_for_testing_info::ChromedriverForTestingInfo;
use webdriver_downloader::driver_impls::chromedriver_info::OfflineVerificationError;
use webdriver_downloader::driver_impls::chromedriver_old_info::ChromedriverOldInfo;
use webdriver_downloader::prelude::*;

/// Downstream crates are generic over the driver, so this has to keep compiling.
async fn install(
    driver: &impl WebdriverDownloadInfo,
    verify: bool,
    max_tries: usize,
) -> Result<(), WebdriverDownloadError> {
    if !driver.is_installed().await {
        if verify {
            driver.download_verify_install(max_tries).await?;
        } else {
            driver.download_install().await?;
        }
    }

    Ok(())
}

#[test]
fn driver_types_implement_download_info() {
    fn assert_download_info<T: WebdriverDownloadInfo>() {}

    assert_download_info::<ChromedriverInfo>();
    assert_download_info::<ChromedriverForTestingInfo>();
    assert_download_info::<ChromedriverOldInfo>();
    assert_download_info::<GeckodriverInfo>();
}

#[test]
fn install_future_is_send() {
    fn assert_send<T: Send>(_: T) {}

    let driver = ChromedriverInfo::new("driver".into(), "browser".into());
    assert_send(install(&driver, true, 1));
}

#[test]
fn errors_work_with_error_handling_crates() {
    fn assert_error<T: Error + Send + Sync + 'static>() {}

    assert_error::<WebdriverDownloadError>();
    assert_error::<UrlError>();
    assert_error::<InstallationError>();
    assert_error::<AddExecutePermissionError>();
    assert_error::<VerificationError>();
    assert_error::<VersionReqError>();
    assert_error::<os_specific::DefaultPathError>();
    assert_error::<OfflineVerificationError>();
}

#[test]
fn chromedriver_info_exposes_its_paths() {
    let mut driver = ChromedriverInfo::new("driver".into(), "browser".into());
    assert_eq!(driver.driver_install_path(), std::path::Path::new("driver"));
    assert_eq!(driver.browser_path(), std::path::Path::new("browser"));

    driver.set_install_path("other".into());
    assert_eq!(driver.driver_install_path(), std::path::Path::new("other"));
}
