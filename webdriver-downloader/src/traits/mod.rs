//! The interface for downloading webdrivers, and the errors it can return.
//!
//! [`webdriver_download_info::WebdriverDownloadInfo`] is the entry point. It's implemented for the
//! drivers in [`crate::driver_impls`] and can't be implemented outside this crate.

pub mod version_req_url_info;

pub mod webdriver_download_info;

pub mod installation_info;
pub mod url_info;
pub mod verification_info;

/// An error from one of this crate's dependencies.
///
/// The concrete type is an implementation detail and can change in any release, so that upgrading
/// a dependency isn't a breaking change. Use its [`Display`](std::fmt::Display) output and
/// [`source`](std::error::Error::source) chain rather than downcasting it.
pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
