//! Traits for the webdriver-downloader crate.
//!
//! This module contains the traits that are used by the library to download,
//! install and verify webdrivers.
//!
//! Implementing [`installation_info::WebdriverInstallationInfo`], [`url_info::WebdriverUrlInfo`] and [`verification_info::WebdriverVerificationInfo`]
//! will allow you to use the [`webdriver_download_info::WebdriverDownloadInfo`] trait's blanket implementation.
//!
//! If the webdriver you want to implement depends on a versioning scheme, you can implement [`version_req_url_info::VersionReqUrlInfo`] instead of [`url_info::WebdriverUrlInfo`].
//!
//! See [`crate::driver_impls`] for how to implement these traits.

pub mod version_req_url_info;

pub mod webdriver_download_info;

pub mod installation_info;
pub mod url_info;
pub mod verification_info;
