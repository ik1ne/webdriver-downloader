# webdriver-downloader
Cli Interface&amp;Library for webdriver download.

## Usage

### CLI

```shell
# Installs chromedriver into %USERPROFILE%/bin/chromedriver.exe
webdriver-downlod.exe

# You can specify path to install driver to. Adding mkdir flag will create the parent directiry.
webdriver-download.exe --mkdir --driver ./drivers/chromedriver.exe

# You can also provide the path to the browser executable, which is useful for downloading webdriver for different channels.
webdriver-download.exe --mkdir --driver ./drivers/chromedriver_dev.exe --browser "C:/Program Files/Google/Chrome Dev/Application/chrome.exe"

```

### Library
#### Downloading library provided driver
```rust
use std::path::PathBuf;

use webdriver_downloader::{download_verify_install, ChromedriverInfo};

#[tokio::main]
async fn main() {
    let driver_info = ChromedriverInfo::new(
        PathBuf::from(r"./webdrivers/chromedriver.exe"),
        PathBuf::from("C:/Program Files/Google/Chrome/Application/chrome.exe"),
    );

    # Tries up to 5 versions of webdrivers.
    download_verify_install(driver_info, 5).await.unwrap();
}
```

#### Implementing downloader for custom driver
You can implement trait `WebdriverUrlInfo, WebdriverInstallationInfo, WebdriverVerificationInfo` for CustomDriverInfo and call `download_verify_install(custom_driver_info, max_attempts)`.