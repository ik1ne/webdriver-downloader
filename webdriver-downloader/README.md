# webdriver-downloader

[![Crates.io][crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/webdriver-downloader.svg
[crates-url]: https://crates.io/crates/webdriver-downloader

Library for webdriver download. Supports chromedriver(both Chrome and Chrome for Testing), geckodriver for Windows, Linux and macOS.

## Usage

### Using library provided driver downloader

```rust
use webdriver_downloader::prelude::*;

#[tokio::main]
async fn main() {
    let driver_info = ChromedriverInfo::new_default().unwrap();

    // Tries up to 5 versions of webdrivers if it is not installed.
    if !driver_info.is_installed().await {
        driver_info.download_verify_install(5).await.unwrap();
    }

    // webdriver is installed.
    // Default installation path is %USERPROFILE%/bin/chromedriver.exe ($HOME/bin/chromedriver for unix family)
}
```

### Supporting another driver

Drivers live in this crate, and `WebdriverDownloadInfo` can't be implemented outside it. To add one, please open a pull
request.

## Notes on runtime

The library uses `fantoccini` to verify installed driver.
Since `fantoccini` requires `tokio` as runtime, you need to use `tokio` as runtime to use library provided driver downloader.
