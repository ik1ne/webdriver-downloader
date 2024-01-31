use std::ffi::OsStr;

use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

use webdriver_downloader::prelude::*;

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
