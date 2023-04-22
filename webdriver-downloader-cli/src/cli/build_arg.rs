use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::{arg, command, value_parser, ArgMatches};

use super::{Args, DriverType};

pub(super) fn get_args() -> Result<Args> {
    let matches = command!()
        .arg(
            arg!(--type <TYPE> "driver type")
                .default_value("chrome")
                .value_parser(["chrome", "gecko"]),
        )
        .arg(
            arg!(--driver <PATH> "driver path")
                .default_value("-")
                .value_parser(value_parser!(PathBuf))
                .help("path to install driver to. Defaults to HOME_DIR/bin/driver_name.exe"),
        )
        .arg(
            arg!(--browser <PATH> "browser path")
                .default_value("-")
                .value_parser(value_parser!(PathBuf))
                .help("path to browser executable. Defaults to default install location."),
        )
        .arg(arg!(--mkdir).help("make directory to driver path"))
        .arg(arg!(--reinstall).help("force reinstall even if driver is already installed"))
        .get_matches();

    let driver_type = get_driver_type(&matches);
    let driver_install_path = get_driver_install_path(&matches, driver_type)?;
    let browser_path = get_browser_path(&matches, driver_type)?;
    let mkdir = get_mkdir(&matches);
    let reinstall = get_reinstall(&matches);

    Ok(Args {
        driver_type,
        driver_install_path,
        browser_path,
        mkdir,
        reinstall,
    })
}

fn get_driver_type(matches: &ArgMatches) -> DriverType {
    let browser_type = matches
        .get_one::<String>("type")
        .expect("\"type\" arg is empty");

    match browser_type.as_str() {
        "chrome" => DriverType::Chrome,
        "gecko" => DriverType::Gecko,
        _ => panic!("Unexpected argument value of \"type\". {:?}", browser_type),
    }
}

fn get_driver_install_path(matches: &ArgMatches, driver_type: DriverType) -> Result<PathBuf> {
    let driver_install_path = matches
        .get_one::<PathBuf>("driver")
        .expect("\"driver\" arg is empty");

    if driver_install_path == Path::new("-") {
        driver_type
            .default_driver_install_path()
            .map_err(|e| e.into())
    } else {
        Ok(driver_install_path.clone())
    }
}

fn get_browser_path(matches: &ArgMatches, driver_type: DriverType) -> Result<PathBuf> {
    let browser_path = matches
        .get_one::<PathBuf>("browser")
        .expect("\"browser\" arg is empty");

    if browser_path == Path::new("-") {
        driver_type.default_browser_path().map_err(|e| e.into())
    } else {
        Ok(browser_path.clone())
    }
}

fn get_mkdir(matches: &ArgMatches) -> bool {
    matches.get_flag("mkdir")
}

fn get_reinstall(matches: &ArgMatches) -> bool {
    matches.get_flag("reinstall")
}
