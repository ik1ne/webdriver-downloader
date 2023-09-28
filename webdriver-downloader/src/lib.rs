#![deny(rustdoc::broken_intra_doc_links)] // error if there are broken intra-doc links

//! A library for downloading and installing webdrivers.
//!
//! This library provides a generic interface for downloading, installing and
//! verifying webdrivers. It also provides implementations for the most common
//! webdrivers.
//!
//! # Examples
//!
//! ## Downloading and installing chromedriver
//!
//! ```no_run
//! use webdriver_downloader::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), WebdriverDownloadError> {
//!     let chromedriver_info = ChromedriverOldInfo::new_default()?;
//!
//!     if !chromedriver_info.is_installed().await {
//!        chromedriver_info.download_verify_install(5).await?;
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! This will download the latest version of chromedriver, verify it, and install
//! it to the default location.
//! See [`default_install_path`](os_specific::chromedriver_old::default_driver_path)s for default install locations.
//!
//! # Implementing your own webdriver
//!
//! See [`traits`] for more information on how to implement your own webdriver.
pub mod driver_impls;
pub mod os_specific;
pub mod traits;

/// Re-exported driver implementations, traits, errors and os_specific data.
///
/// This module is intended to be used as a prelude, and contains all the
/// necessary imports for using library provided driver implementations.
pub mod prelude {
    pub use crate::driver_impls::{
        chromedriver_old_info::ChromedriverOldInfo, geckodriver_info::GeckodriverInfo,
    };
    pub use crate::os_specific;
    pub use crate::traits::installation_info::{
        AddExecutePermissionError, InstallationError, WebdriverInstallationInfo,
    };
    pub use crate::traits::url_info::{UrlError, WebdriverUrlInfo, WebdriverVersionUrl};
    pub use crate::traits::verification_info::{VerificationError, WebdriverVerificationInfo};
    pub use crate::traits::version_req_url_info::{VersionReqError, VersionReqUrlInfo};
    pub use crate::traits::webdriver_download_info::{
        WebdriverDownloadError, WebdriverDownloadInfo,
    };
}
