use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;

pub fn check_driver_install_path<P: AsRef<Path>>(path: &P, mkdir: bool) -> Result<()> {
    let parent = path
        .as_ref()
        .parent()
        .with_context(|| "Failed to get parent directory of driver install path.")?;

    if mkdir {
        fs::create_dir_all(parent)
            .with_context(|| "Failed to create parent directory of driver install path.")?;
    }

    if parent.exists() {
        Ok(())
    } else {
        bail!("Failed to get parent directory of driver install path.")
    }
}

pub fn check_browser<P: AsRef<Path>>(path: &P) -> Result<()> {
    if path.as_ref().exists() {
        Ok(())
    } else {
        bail!("Failed to find browser executable.")
    }
}
