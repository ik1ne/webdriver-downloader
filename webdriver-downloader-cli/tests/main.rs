use std::ffi::OsStr;
use std::path::PathBuf;

use assert_cmd::Command;
use assert_fs::prelude::*;
use assert_fs::TempDir;
use predicates::prelude::*;
use predicates::str::{contains, ends_with};

use webdriver_downloader::prelude::*;

const CHROMEDRIVER_BIN: &str = os_specific::chromedriver::DRIVER_EXECUTABLE_NAME;

fn download_driver_to_temp_dir() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let mut driver_path = temp_dir.to_path_buf();
    driver_path.push(CHROMEDRIVER_BIN);

    let mut cmd = Command::cargo_bin("webdriver-downloader").unwrap();
    let assert = cmd
        .args([OsStr::new("--driver"), driver_path.as_os_str()].iter())
        .assert();

    assert.success();
    temp_dir
        .child(CHROMEDRIVER_BIN)
        .assert(predicate::path::exists());

    (temp_dir, driver_path)
}

// Tests for chromedriver, which also checks overall functionality.
#[test]
fn test_passes_no_mkdir() {
    let _ = download_driver_to_temp_dir();
}

/// Test for skip_verification flag
#[test]
fn test_passes_skip_verification() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut driver_path = temp_dir.to_path_buf();
    driver_path.push(CHROMEDRIVER_BIN);

    let mut cmd = Command::cargo_bin("webdriver-downloader").unwrap();
    let assert = cmd
        .args(
            [
                OsStr::new("--driver"),
                driver_path.as_os_str(),
                OsStr::new("--skip-verify"),
            ]
            .iter(),
        )
        .assert();

    assert.success();
    temp_dir
        .child(CHROMEDRIVER_BIN)
        .assert(predicate::path::exists());
}

/// Test for mkdir flag
#[test]
fn test_passes_mkdir() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut driver_path = temp_dir.to_path_buf();
    driver_path.push("new_dir");
    driver_path.push(CHROMEDRIVER_BIN);

    let mut cmd = Command::cargo_bin("webdriver-downloader").unwrap();
    let assert = cmd
        .args(
            [
                OsStr::new("--driver"),
                driver_path.as_os_str(),
                OsStr::new("--mkdir"),
            ]
            .iter(),
        )
        .assert();

    assert.success();
    temp_dir
        .child(format!("new_dir/{}", CHROMEDRIVER_BIN))
        .assert(predicate::path::exists());
}

// Test for reinstall flag
/// Test for the case where the driver is already installed and reinstall flag is not set.
#[tokio::test]
async fn test_existing_driver() {
    let (temp_dir, driver_path) = download_driver_to_temp_dir();

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let mut cmd = Command::cargo_bin("webdriver-downloader").unwrap();
    let assert = cmd
        .args([OsStr::new("--driver"), driver_path.as_os_str()].iter())
        .assert();

    assert
        .success()
        .stdout(contains("Driver already installed.\n"));

    temp_dir
        .child(CHROMEDRIVER_BIN)
        .assert(predicate::path::exists());
}

/// Test for the case where the driver is already installed and reinstall flag is set.
#[tokio::test]
async fn test_reinstall() {
    let (temp_dir, driver_path) = download_driver_to_temp_dir();

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let mut cmd = Command::cargo_bin("webdriver-downloader").unwrap();
    let assert = cmd
        .args(
            [
                OsStr::new("--driver"),
                driver_path.as_os_str(),
                OsStr::new("--reinstall"),
            ]
            .iter(),
        )
        .assert();

    assert
        .success()
        .stdout(ends_with("Driver installed successfully.\n"));

    temp_dir
        .child(CHROMEDRIVER_BIN)
        .assert(predicate::path::exists());
}

// Testcases for failures

/// Test for num_tries
#[test]
fn test_fails_0_tries() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut driver_path = temp_dir.to_path_buf();
    driver_path.push(CHROMEDRIVER_BIN);

    let mut cmd = Command::cargo_bin("webdriver-downloader").unwrap();
    let assert = cmd
        .args(
            [
                OsStr::new("--driver"),
                driver_path.as_os_str(),
                OsStr::new("--tries"),
                OsStr::new("0"),
            ]
            .iter(),
        )
        .assert();

    assert.failure();
}

#[test]
fn test_fails_negative_tries() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut driver_path = temp_dir.to_path_buf();
    driver_path.push(CHROMEDRIVER_BIN);

    let mut cmd = Command::cargo_bin("webdriver-downloader").unwrap();
    let assert = cmd
        .args(
            [
                OsStr::new("--driver"),
                driver_path.as_os_str(),
                OsStr::new("--tries"),
                OsStr::new("-1"),
            ]
            .iter(),
        )
        .assert();

    assert.failure();
}

/// Test for no parent directory when mkdir flag is not set
#[test]
fn test_fails_no_mkdir_and_no_dir() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut driver_path = temp_dir.to_path_buf();
    driver_path.push("new_dir");
    driver_path.push(CHROMEDRIVER_BIN);

    let mut cmd = Command::cargo_bin("webdriver-downloader").unwrap();
    let assert = cmd
        .args([OsStr::new("--driver"), driver_path.as_os_str()].iter())
        .assert();

    assert.failure();
    temp_dir.child("new_dir").assert(predicate::path::missing());
}

/// Test for browser path not found
#[test]
fn test_fails_no_browser() {
    let mut cmd = Command::cargo_bin("webdriver-downloader").unwrap();
    let assert = cmd
        .args([OsStr::new("--browser"), OsStr::new("no_browser.exe")].iter())
        .assert();

    assert.failure();
}

// Tests for other drivers.

// geckodriver
const GECKODRIVER_BIN: &str = os_specific::geckodriver::DRIVER_EXECUTABLE_NAME;

#[test]
fn test_geckodriver() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut driver_path = temp_dir.to_path_buf();
    driver_path.push(GECKODRIVER_BIN);

    let mut cmd = Command::cargo_bin("webdriver-downloader").unwrap();
    let assert = cmd
        .args(
            [
                OsStr::new("--driver"),
                driver_path.as_os_str(),
                OsStr::new("--type"),
                OsStr::new("gecko"),
            ]
            .iter(),
        )
        .assert();

    assert.success();
    temp_dir
        .child(GECKODRIVER_BIN)
        .assert(predicate::path::exists());
}
