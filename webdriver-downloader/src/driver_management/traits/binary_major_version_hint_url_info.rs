use std::cmp::Ordering;

use anyhow::Result;
use async_trait::async_trait;
use semver::Version;

use super::WebdriverUrlInfo;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct VersionUrl {
    pub driver_version: Version,
    pub url: String,
}

#[async_trait]
pub trait BinaryMajorVersionHintUrlInfo: WebdriverUrlInfo {
    /// Version hint. Used by [`BinaryMajorVersionHintUrlInfo::compare_version`].
    fn binary_version(&self) -> Option<Version>;

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

#[cfg(test)]
mod test {
    use anyhow::Result;
    use async_trait::async_trait;
    use semver::Version;
    use std::cmp::Ordering;

    use crate::traits::{BinaryMajorVersionHintUrlInfo, VersionUrl};
    use crate::WebdriverUrlInfo;

    struct MockBinaryMajorVersionHintUrlInfo {
        version_hint: Option<Version>,
        version_urls: Vec<VersionUrl>,
    }

    #[async_trait]
    impl BinaryMajorVersionHintUrlInfo for MockBinaryMajorVersionHintUrlInfo {
        fn binary_version(&self) -> Option<Version> {
            self.version_hint.clone()
        }

        async fn driver_version_urls(&self) -> Result<Vec<VersionUrl>> {
            Ok(self.version_urls.clone())
        }
    }

    fn create_dummy_version_info(driver_version: Version) -> VersionUrl {
        let url = driver_version.to_string();
        VersionUrl {
            driver_version,
            url,
        }
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
            mock_info.driver_urls(5).await.unwrap(),
            vec![
                "3.0.0".to_string(),
                "2.0.0".to_string(),
                "1.0.0".to_string(),
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
            mock_info.driver_urls(2).await.unwrap(),
            vec!["3.0.0".to_string(), "2.0.0".to_string(),]
        )
    }
}
