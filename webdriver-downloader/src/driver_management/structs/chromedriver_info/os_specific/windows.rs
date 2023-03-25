use crate::structs::chromedriver_info::BinaryVersionError;

pub const ZIPFILE_NAME_RE: &str = r"<Key>([0-9\.]*?)/chromedriver_win32.zip</Key>";
pub const DRIVER_NAME_IN_ARCHIVE: &str = "chromedriver.exe";

pub fn build_url(version_string: &str) -> String {
    format!(
        "https://chromedriver.storage.googleapis.com/{}/chromedriver_win32.zip",
        version_string
    )
}

pub fn binary_version(browser_path: &Path) -> Result<Version, BinaryVersionError> {
    let mut child = std::process::Command::new("powershell");

    let mut command = OsString::from("(Get-Item \"");
    command.push(browser_path);
    command.push("\").VersionInfo.FileVersion");

    child.arg("-command").arg(command);

    let output = child.output()?;
    lenient_semver::parse(&String::from_utf8_lossy(&output.stdout)).map_err(|e| e.owned().into())
}
