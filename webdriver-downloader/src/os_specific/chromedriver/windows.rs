use std::path::Path;

use semver::Version;

use crate::traits::version_req_url_info::VersionReqError;

pub const DRIVER_EXECUTABLE_NAME: &str = "chromedriver.exe";

pub fn binary_version(browser_path: &Path) -> Result<Version, VersionReqError> {
    let mut child = std::process::Command::new("powershell");

    let mut command = OsString::from("(Get-Item \"");
    command.push(browser_path);
    command.push("\").VersionInfo.FileVersion");

    child.arg("-command").arg(command);

    let output = child.output()?;
    lenient_semver::parse(&String::from_utf8_lossy(&output.stdout)).map_err(|e| e.owned().into())
}
