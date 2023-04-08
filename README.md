# webdriver-downloader
Cli Interface&amp;Library for webdriver download. Supports chromedriver, geckodriver for Windows, Linux and macOS.

## Usage

### CLI

```shell
# Install webdriver-downloader-cli
cargo install webdriver-downloader-cli

# Installs chromedriver into %USERPROFILE%/bin/chromedriver.exe ($HOME/bin/chromedriver for unix family)
webdriver-download.exe

# You can specify path to install driver to. Adding mkdir flag will create the parent directiry.
webdriver-download.exe --mkdir --driver ./drivers/chromedriver.exe

# You can also provide the path to the browser executable, which is useful for downloading webdriver for different channels.
webdriver-download.exe --mkdir --driver ./drivers/chromedriver_dev.exe --browser "C:/Program Files/Google/Chrome Dev/Application/chrome.exe"



# Supported driver types are "chrome", "gecko".
webdriver-download.exe --type gecko
```

### Library
#### Downloading library provided driver
```rust
use std::path::PathBuf;

use webdriver_downloader::structs::ChromedriverInfo;

#[tokio::main]
async fn main() {
    let driver_info = ChromedriverInfo::new(
        PathBuf::from("./webdrivers/chromedriver.exe"),
        PathBuf::from("C:/Program Files/Google/Chrome/Application/chrome.exe"),
    );

    // Tries up to 5 versions of webdrivers.
    driver_info.download_verify_install(5).await.unwrap();
}
```

#### Implementing downloader for custom driver
You can implement trait `WebdriverUrlInfo, WebdriverInstallationInfo, WebdriverVerificationInfo` for CustomDriverInfo and call `custom_driver_info.download_verify_install(max_attempts)`.
