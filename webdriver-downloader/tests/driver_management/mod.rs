#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use anyhow::anyhow;
    use mockall::predicate::eq;
    use semver::Version;

    use webdriver_downloader::common::url_info::WebdriverVersionUrl;
    use webdriver_downloader::WebdriverInfo;

    use crate::MockWebdriverInfo;

    #[tokio::test]
    async fn fails_when_0_max_tries() {
        let mut mock = MockWebdriverInfo::new();
        mock.expect_version_urls()
            .with(eq(0))
            .return_once(|_limit| Ok(vec![]));
        let result = mock.download_verify_install(0).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn passes_when_one_version_passes() {
        let mut mock = MockWebdriverInfo::new();
        let version_count = 5;

        let dummy_version_url = WebdriverVersionUrl {
            version_req: Default::default(),
            webdriver_version: Version::new(0, 0, 0),
            url: Default::default(),
        };
        let urls = vec![dummy_version_url; version_count];

        mock.expect_version_urls()
            .with(eq(version_count))
            .return_once(|_limit| Ok(urls));

        mock.expect_download_in_tempdir()
            .returning(|_url: String, _dir| Ok(Default::default()));

        let mut verification_count = 0;
        mock.expect_verify_driver()
            .returning(move |_driver_path: &PathBuf| {
                verification_count += 1;
                println!("Verification count: {}", verification_count);
                if verification_count == version_count {
                    Ok(())
                } else {
                    Err(anyhow!("Verification failed.").into())
                }
            });

        mock.expect_install_driver()
            .returning(|_path: &PathBuf| Ok(()));

        let result = mock.download_verify_install(version_count).await;
        assert!(result.is_ok());
    }
}
