//! OS-specific constants and functions.
//!
//! This module contains constants and functions that are specific to a particular
//! operating system.

pub mod chromedriver;
pub mod geckodriver;

/// Errors that can occur when getting the default path for a webdriver.
#[derive(thiserror::Error, Debug)]
pub enum DefaultPathError {
    #[error("Failed to get home directory")]
    HomeDir,
    #[error("Failed to get Program Files directory")]
    ProgramFiles(#[from] std::env::VarError),
    /// Failed to run `which` command. If which failed due to a missing binary, [`BinaryNotFound`](DefaultPathError::BinaryNotFound) will be returned instead.
    #[error("Failed to run command")]
    Which(#[from] which::Error),
    #[error("Failed to get browser binary path")]
    BinaryNotFound,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
