# webdriver-downloader

Cli Interface for webdriver download. Supports chromedriver, geckodriver for Windows, Linux and macOS.

## Usage

```shell
# Install webdriver-downloader-cli
cargo install webdriver-downloader-cli

# Installs chromedriver into %USERPROFILE%/bin/chromedriver.exe ($HOME/bin/chromedriver for unix family)
webdriver-downloader.exe

# You can specify path to install driver to. Adding mkdir flag will create the parent directory.
webdriver-downloader.exe --mkdir --driver ./drivers/chromedriver.exe

# You can also provide the path to the browser executable, which is useful for downloading webdriver for different channels.
webdriver-downloader.exe --mkdir --driver ./drivers/chromedriver_dev.exe --browser "C:/Program Files/Google/Chrome Dev/Application/chrome.exe"


# Supported driver types are "chrome", "gecko".
webdriver-downloader.exe --type gecko
```