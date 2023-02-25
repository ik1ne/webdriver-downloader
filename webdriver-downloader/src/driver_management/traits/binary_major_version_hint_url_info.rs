use std::cmp::Ordering;

use anyhow::Result;
use async_trait::async_trait;
use semver::Version;

use super::WebdriverUrlInfo;

pub struct VersionUrl {
    pub driver_version: Version,
    pub url: String,
}

#[async_trait]
pub trait BinaryMajorVersionHintUrlInfo: WebdriverUrlInfo {
    /// Version hint. used by [compare_version](BinaryMajorVersionHintUrlInfo::driver_urls).
    fn binary_version(&self) -> Option<Version>;

    /// Compares versions based on `version_hint`.
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
}

#[async_trait]
impl<T> WebdriverUrlInfo for T
where
    T: BinaryMajorVersionHintUrlInfo + Sync,
{
    async fn driver_urls(&self, limit: usize) -> Result<Vec<String>> {
        let mut url_infos = self.driver_version_urls().await?;

        if let Some(version_hint) = self.binary_version() {
            url_infos.sort_by(|left, right| {
                Self::compare_version(&version_hint, &right.driver_version, &left.driver_version)
            });
        } else {
            url_infos.sort_by(|left, right| right.driver_version.cmp(&left.driver_version))
        }

        Ok(url_infos
            .into_iter()
            .take(limit)
            .map(|version_url| version_url.url)
            .collect())
    }
}
