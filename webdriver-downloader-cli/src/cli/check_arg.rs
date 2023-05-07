use std::fs;

use anyhow::{bail, Context, Result};

use crate::cli::Args;

pub(crate) fn check_driver_install_path(args: &Args) -> Result<()> {
    let parent = args
        .driver_install_path
        .parent()
        .with_context(|| "Failed to get parent directory of driver install path.")?;

    if args.mkdir {
        fs::create_dir_all(parent)
            .with_context(|| "Failed to create parent directory of driver install path.")?;
    }

    if parent.exists() {
        Ok(())
    } else {
        bail!("Failed to get parent directory of driver install path.")
    }
}

pub(crate) fn check_browser(args: &Args) -> Result<()> {
    if args.browser_path.exists() {
        Ok(())
    } else {
        bail!("Failed to find browser executable.")
    }
}

pub(crate) fn check_tries(args: &Args) -> Result<()> {
    if args.skip_verification {
        return Ok(());
    }

    if args.num_tries > 0 {
        Ok(())
    } else {
        bail!("Number of tries must be greater than 0.")
    }
}
