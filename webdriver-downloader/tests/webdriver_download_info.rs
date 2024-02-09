use std::sync::{Arc, Mutex};

use anyhow::Result;
use semver::Version;

use common::MockWebdriverDownloadInfo;
use webdriver_downloader::prelude::*;

mod common;

#[tokio::test]
async fn fails_when_0_max_tries() {
    let mut mock = MockWebdriverDownloadInfo::new();
    mock.version_urls = Some(vec![]);

    let result = mock.download_verify_install(0).await;
    result.unwrap_err();
}

#[tokio::test]
async fn passes_when_one_version_passes() -> Result<()> {
    let mut mock = MockWebdriverDownloadInfo::new();
    let version_count = 5;

    let dummy_version_url = WebdriverVersionUrl {
        version_req: Default::default(),
        webdriver_version: Version::new(0, 0, 0),
        url: Default::default(),
    };
    let urls = vec![dummy_version_url; version_count];

    mock.version_urls = Some(urls);
    mock.download_in_tempdir = Some(Default::default());
    mock.verify_driver = Arc::new(Mutex::new(vec![false, false, false, false, true]));
    mock.install_driver = Arc::new(Mutex::new(vec![true; version_count]));

    mock.download_verify_install(version_count).await?;

    Ok(())
}

#[tokio::test]
async fn fails_when_all_versions_fail() {
    let mut mock = MockWebdriverDownloadInfo::new();
    let version_count = 5;

    let dummy_version_url = WebdriverVersionUrl {
        version_req: Default::default(),
        webdriver_version: Version::new(0, 0, 0),
        url: Default::default(),
    };
    let urls = vec![dummy_version_url; version_count];

    mock.version_urls = Some(urls);
    mock.download_in_tempdir = Some(Default::default());
    mock.verify_driver = Arc::new(Mutex::new(vec![false; 5]));

    let result = mock.download_verify_install(version_count).await;
    result.unwrap_err();
}

#[tokio::test]
async fn fails_when_installation_fails() {
    let mut mock = MockWebdriverDownloadInfo::new();
    let version_count = 5;

    let dummy_version_url = WebdriverVersionUrl {
        version_req: Default::default(),
        webdriver_version: Version::new(0, 0, 0),
        url: Default::default(),
    };
    let urls = vec![dummy_version_url; version_count];

    mock.version_urls = Some(urls);
    mock.download_in_tempdir = Some(Default::default());
    mock.verify_driver = Arc::new(Mutex::new(vec![true; 5]));
    mock.install_driver = Arc::new(Mutex::new(vec![false; 5]));

    mock.download_verify_install(version_count)
        .await
        .unwrap_err();
}
