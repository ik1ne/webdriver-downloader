use std::cmp::Ordering;
use std::collections::BTreeMap;

use async_trait::async_trait;
use semver::Version;

use crate::traits::url_info::WebdriverVersionUrl;

use super::url_info::{UrlError, WebdriverUrlInfo};

#[derive(thiserror::Error, Debug)]
pub enum VersionReqError {
    #[error("Failed to capture regex from string: {0}")]
    RegexError(String),
    #[error(transparent)]
    ParseVersion(#[from] lenient_semver::parser::OwnedError),
    #[error("Failed to execute binary: {0}")]
    Execute(#[from] std::io::Error),
    #[error(transparent)]
    ParseVersionReq(#[from] semver::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Provides information for determining which url to download.
/// This trait uses the version to sort the urls to download.
#[async_trait]
pub trait VersionReqUrlInfo: WebdriverUrlInfo {
    /// Version hint. Used by [`VersionReqUrlInfo::compare_driver`].
    fn binary_version(&self) -> Result<Version, VersionReqError>;

    /// Compares webdrivers based on [`binary_version`](VersionReqUrlInfo::binary_version).
    ///
    /// Prioritizes same major version to version_hint, and then latest version.
    fn compare_driver(
        binary_version: &Version,
        left: &WebdriverVersionUrl,
        right: &WebdriverVersionUrl,
    ) -> Ordering {
        let left_match = left.version_req.matches(binary_version);
        let right_match = right.version_req.matches(binary_version);
        match (left_match, right_match) {
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            _ => left.webdriver_version.cmp(&right.webdriver_version),
        }
    }

    /// [`WebdriverVersionUrl`]s, probably parsed from driver's download page.
    async fn driver_version_urls(&self) -> Result<Vec<WebdriverVersionUrl>, UrlError>;
}

#[async_trait]
impl<T> WebdriverUrlInfo for T
where
    T: VersionReqUrlInfo + Sync,
{
    async fn version_urls(&self, limit: usize) -> Result<Vec<WebdriverVersionUrl>, UrlError> {
        let url_infos = self.driver_version_urls().await?;

        let cmp: Box<dyn Fn(&WebdriverVersionUrl, &WebdriverVersionUrl) -> Ordering> =
            match self.binary_version() {
                Ok(version_hint) => Box::new(
                    move |left: &WebdriverVersionUrl, right: &WebdriverVersionUrl| {
                        left.version_req.comparators.get(0).unwrap().to_string();
                        Self::compare_driver(&version_hint, left, right)
                    },
                ),
                Err(e) => {
                    println!("Failed to parse binary version: {}", e);

                    Box::new(|left: &WebdriverVersionUrl, right: &WebdriverVersionUrl| {
                        left.webdriver_version.cmp(&right.webdriver_version)
                    })
                }
            };

        let mut semver_map: BTreeMap<(u64, u64), WebdriverVersionUrl> = BTreeMap::new();

        for version_url in url_infos {
            let key = (
                version_url.webdriver_version.major,
                version_url.webdriver_version.minor,
            );
            if let Some(existing_version_url) = semver_map.get_mut(&key) {
                if cmp(&version_url, existing_version_url) == Ordering::Greater {
                    *existing_version_url = version_url;
                }
            } else {
                semver_map.insert(key, version_url);
            }
        }

        let mut versions = semver_map.into_values().collect::<Vec<_>>();

        versions.sort_by(|l, r| cmp(r, l));

        if versions.len() > limit {
            versions.truncate(limit);
        }

        Ok(versions)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use async_trait::async_trait;
    use semver::{Version, VersionReq};

    use crate::traits::url_info::{UrlError, WebdriverUrlInfo};
    use crate::traits::version_req_url_info::{
        VersionReqError, VersionReqUrlInfo, WebdriverVersionUrl,
    };

    struct MockBinaryMajorVersionHintUrlInfo {
        version_hint: Option<Version>,
        version_urls: Vec<WebdriverVersionUrl>,
    }

    #[async_trait]
    impl VersionReqUrlInfo for MockBinaryMajorVersionHintUrlInfo {
        fn binary_version(&self) -> Result<Version, VersionReqError> {
            self.version_hint
                .clone()
                .ok_or(VersionReqError::Other(anyhow::anyhow!("No version hint")))
        }

        async fn driver_version_urls(&self) -> Result<Vec<WebdriverVersionUrl>, UrlError> {
            Ok(self.version_urls.clone())
        }
    }

    fn dummy_version_info(version: Version) -> WebdriverVersionUrl {
        let version_string = version.to_string();
        WebdriverVersionUrl {
            version_req: VersionReq::parse(&format!("^{}", version_string)).unwrap(),
            url: Default::default(),
            webdriver_version: Version::parse(&version_string).unwrap(),
        }
    }

    #[test]
    fn compare_version_prioritizes_same_major_version() {
        assert_eq!(
            MockBinaryMajorVersionHintUrlInfo::compare_driver(
                &Version::parse("2.0.0").unwrap(),
                &dummy_version_info(Version::new(3, 0, 0),),
                &dummy_version_info(Version::new(2, 0, 0),),
            ),
            Ordering::Less
        );
    }

    #[tokio::test]
    async fn compare_successes_without_version_hint() {
        let version_urls = vec![
            dummy_version_info(Version::new(1, 0, 0)),
            dummy_version_info(Version::new(3, 0, 0)),
            dummy_version_info(Version::new(2, 0, 0)),
        ];

        let mock_info = MockBinaryMajorVersionHintUrlInfo {
            version_hint: None,
            version_urls,
        };
        assert_eq!(
            mock_info.version_urls(5).await.unwrap(),
            vec![
                dummy_version_info(Version::new(3, 0, 0)),
                dummy_version_info(Version::new(2, 0, 0)),
                dummy_version_info(Version::new(1, 0, 0)),
            ]
        )
    }

    #[tokio::test]
    async fn driver_urls_limit_works() {
        let version_urls = vec![
            dummy_version_info(Version::new(1, 0, 0)),
            dummy_version_info(Version::new(3, 0, 0)),
            dummy_version_info(Version::new(2, 0, 0)),
        ];

        let mock_info = MockBinaryMajorVersionHintUrlInfo {
            version_hint: None,
            version_urls,
        };

        assert_eq!(
            mock_info.version_urls(2).await.unwrap(),
            vec![
                dummy_version_info(Version::new(3, 0, 0)),
                dummy_version_info(Version::new(2, 0, 0)),
            ]
        )
    }
}
