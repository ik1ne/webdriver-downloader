use std::cmp::Ordering;
use std::collections::BTreeMap;

use async_trait::async_trait;
use semver::Version;

use crate::common::url_info::VersionUrl;

use super::url_info::{UrlError, WebdriverUrlInfo};

#[derive(thiserror::Error, Debug)]
pub enum BinaryVersionError {
    #[error("Failed to capture regex from string: {0}")]
    RegexError(String),
    #[error("Failed to parse binary version: {0}")]
    Parse(#[from] lenient_semver::parser::OwnedError),
    #[error("Failed to execute binary: {0}")]
    Execute(#[from] std::io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Provides information for determining which url to download.
/// This trait uses the binary version to sort the urls to download.
#[async_trait]
pub trait BinaryVersionHintUrlInfo: WebdriverUrlInfo {
    /// Version hint. Used by [`BinaryVersionHintUrlInfo::compare_version`].
    fn binary_version(&self) -> Result<Version, BinaryVersionError>;

    /// Compares versions based on `binary_version`.
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
    async fn driver_version_urls(&self) -> Result<Vec<VersionUrl>, UrlError>;
}

#[async_trait]
impl<T> WebdriverUrlInfo for T
where
    T: BinaryVersionHintUrlInfo + Sync,
{
    async fn version_urls(&self, limit: usize) -> Result<Vec<VersionUrl>, UrlError> {
        let url_infos = self.driver_version_urls().await?;

        let cmp: Box<dyn Fn(&VersionUrl, &VersionUrl) -> Ordering> = match self.binary_version() {
            Ok(version_hint) => Box::new(move |left: &VersionUrl, right: &VersionUrl| {
                Self::compare_version(&version_hint, &left.version, &right.version)
            }),
            Err(e) => {
                println!("Failed to parse binary version: {}", e);

                Box::new(|left: &VersionUrl, right: &VersionUrl| left.version.cmp(&right.version))
            }
        };

        let mut major_version_map: BTreeMap<u64, VersionUrl> = BTreeMap::new();

        for version_url in url_infos {
            if let Some(existing_version_url) =
                major_version_map.get_mut(&version_url.version.major)
            {
                if cmp(&version_url, existing_version_url) == Ordering::Greater {
                    *existing_version_url = version_url;
                }
            } else {
                major_version_map.insert(version_url.version.major, version_url);
            }
        }

        let mut keys_descending: Vec<u64> = major_version_map.keys().copied().collect();
        keys_descending.reverse();

        Ok(keys_descending
            .into_iter()
            .filter_map(|key| major_version_map.remove(&key))
            .take(limit)
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use async_trait::async_trait;
    use semver::Version;

    use crate::common::binary_version_hint_url_info::{
        BinaryVersionError, BinaryVersionHintUrlInfo, VersionUrl,
    };
    use crate::common::url_info::{UrlError, WebdriverUrlInfo};

    struct MockBinaryMajorVersionHintUrlInfo {
        version_hint: Option<Version>,
        version_urls: Vec<VersionUrl>,
    }

    #[async_trait]
    impl BinaryVersionHintUrlInfo for MockBinaryMajorVersionHintUrlInfo {
        fn binary_version(&self) -> Result<Version, BinaryVersionError> {
            self.version_hint
                .clone()
                .ok_or(BinaryVersionError::Other(anyhow::anyhow!(
                    "No version hint"
                )))
        }

        async fn driver_version_urls(&self) -> Result<Vec<VersionUrl>, UrlError> {
            Ok(self.version_urls.clone())
        }
    }

    fn create_dummy_version_info(version: Version) -> VersionUrl {
        let url = version.to_string();
        VersionUrl { version, url }
    }

    #[test]
    fn compare_version_prioritizes_same_major_version() {
        assert_eq!(
            MockBinaryMajorVersionHintUrlInfo::compare_version(
                &Version::new(2, 0, 0),
                &Version::new(3, 0, 0),
                &Version::new(2, 0, 0),
            ),
            Ordering::Less
        );
    }

    #[tokio::test]
    async fn compare_successes_without_version_hint() {
        let version_urls = vec![
            create_dummy_version_info(Version::new(1, 0, 0)),
            create_dummy_version_info(Version::new(3, 0, 0)),
            create_dummy_version_info(Version::new(2, 0, 0)),
        ];

        let mock_info = MockBinaryMajorVersionHintUrlInfo {
            version_hint: None,
            version_urls,
        };
        assert_eq!(
            mock_info.version_urls(5).await.unwrap(),
            vec![
                create_dummy_version_info(Version::new(3, 0, 0)),
                create_dummy_version_info(Version::new(2, 0, 0)),
                create_dummy_version_info(Version::new(1, 0, 0)),
            ]
        )
    }

    #[tokio::test]
    async fn driver_urls_limit_works() {
        let version_urls = vec![
            create_dummy_version_info(Version::new(1, 0, 0)),
            create_dummy_version_info(Version::new(3, 0, 0)),
            create_dummy_version_info(Version::new(2, 0, 0)),
        ];

        let mock_info = MockBinaryMajorVersionHintUrlInfo {
            version_hint: None,
            version_urls,
        };

        assert_eq!(
            mock_info.version_urls(2).await.unwrap(),
            vec![
                create_dummy_version_info(Version::new(3, 0, 0)),
                create_dummy_version_info(Version::new(2, 0, 0)),
            ]
        )
    }
}
