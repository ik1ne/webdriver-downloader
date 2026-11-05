//! OS-specific constants and functions.
//!
//! This module contains constants and functions that are specific to a particular
//! operating system.

pub mod chromedriver;
pub mod chromedriver_for_testing;
pub mod chromedriver_old;
pub mod geckodriver;

use crate::traits::BoxError;

/// Errors that can occur when getting the default path for a webdriver.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum DefaultPathError {
    #[error("Failed to get home directory")]
    HomeDir,
    #[error("Failed to get Program Files directory")]
    ProgramFiles(#[from] std::env::VarError),
    /// Failed to run `which` command. If which failed due to a missing binary, [`BinaryNotFound`](DefaultPathError::BinaryNotFound) will be returned instead.
    #[error("Failed to run command")]
    Which(#[source] BoxError),
    #[error("Failed to get browser binary path")]
    BinaryNotFound,
}
