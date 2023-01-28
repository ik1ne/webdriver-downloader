use fantoccini::wd::Capabilities;
use std::path::Path;

/// Provides information for verifying an installed driver.
pub trait WebdriverVerificationInfo {
    fn browser_path(&self) -> &Path;
    fn driver_path(&self) -> &Path;
    fn driver_capabilities(&self) -> Capabilities;
}

pub async fn verify_driver<P: AsRef<Path>>(
    driver_path: &P,
    verification_info: &impl WebdriverVerificationInfo,
) -> anyhow::Result<()> {
    todo!()
}
