use std::cmp::Ordering;

use anyhow::Result;
use async_trait::async_trait;
use semver::Version;

pub struct VersionUrl {
    pub driver_version: Version,
    pub url: String,
}

/// Provides information for determining which url to download.
#[async_trait]
pub trait WebdriverUrlInfo {
    /// Version hint based on browser's version.
    fn driver_version_hint(&self) -> Option<Version>;

    /// Compares versions based on version_hint.
    /// Prioritizes same major version to version_hint, and then latest version.
    fn compare_version(version_hint: &Version, left: &Version, right: &Version) -> Ordering {
        let left_match = version_hint.major == left.major;
        let right_match = version_hint.major == right.major;
        match (left_match, right_match) {
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            _ => left.cmp(right),
        }
    }

    /// [`VersionUrl`]s, probably parsed from driver's download page.
    async fn driver_version_urls(&self) -> Result<Vec<VersionUrl>>;

    /// Lists viable driver urls based on [driver_url_infos](WebdriverUrlInfo::driver_version_urls) and limit.
    ///
    /// If [driver_version_hint](WebdriverUrlInfo::driver_version_hint) returns Some, driver version with same major versions will be tried first.
    async fn driver_urls(&self, limit: usize) -> Result<Vec<String>> {
        let mut url_infos = self.driver_version_urls().await?;

        if let Some(version_hint) = self.driver_version_hint() {
            url_infos.sort_by(|left, right| {
                Self::compare_version(&version_hint, &right.driver_version, &left.driver_version)
            });
        } else {
            url_infos.sort_by(|left, right| right.driver_version.cmp(&left.driver_version))
        }

        if url_infos.len() > limit {
            url_infos.drain(limit..);
        }

        Ok(url_infos
            .into_iter()
            .map(|version_url| version_url.url)
            .collect())
    }
}
