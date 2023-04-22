# webdriver-downloader

Library for webdriver download. Supports chromedriver, geckodriver for Windows, Linux and macOS.

## Usage

### Downloading library provided driver

```rust
use webdriver_downloader::prelude::*;

#[tokio::main]
async fn main() {
    let driver_info = ChromedriverInfo::new_default().unwrap();

    // Tries up to 5 versions of webdrivers if it is not installed.
    if !driver_info.is_installed() {
        driver_info.download_verify_install(5).await.unwrap();
    }

    // webdriver is installed.
    // Default installation path is %USERPROFILE%/bin/chromedriver.exe ($HOME/bin/chromedriver for unix family)
}
```

### Implementing downloader for custom driver

By implementing `WebdriverUrlInfo, WebdriverInstallationInfo, WebdriverVerificationInfo`, trait `WebdriverDownloadInfo`
is automatically implemented for `struct CustomDriverInfo`.

Then you can call `custom_driver_info.download_verify_install(max_attempts)`.
