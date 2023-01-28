use anyhow::Result;
use async_trait::async_trait;
use semver::Version;
use std::cmp::Ordering;
use std::ffi::{OsStr, OsString};

pub struct VersionUrl {
    pub driver_version: Version,
    pub url: OsString,
}

/// Provides information for determining which url to download.
#[async_trait]
pub trait WebdriverUrlInfo {
    /// Version hint based on browser's version.
    fn driver_version_hint(&self) -> Option<&OsStr>;

    /// [`VersionUrl`]s, probably parsed from driver's download page.
    async fn driver_url_infos(&self) -> Result<Vec<VersionUrl>>;
    async fn driver_urls(&self, limit: usize) -> Result<Vec<OsString>> {
        todo!()
    }
}

fn compare_version(version_hint: &OsStr, left: &VersionUrl, right: &VersionUrl) -> Ordering {
    todo!()
}
