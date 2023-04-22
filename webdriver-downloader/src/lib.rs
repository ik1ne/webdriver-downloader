pub mod driver_impls;
pub mod os_specific;
pub mod traits;

pub mod prelude {
    pub use crate::driver_impls::{
        chromedriver_info::ChromedriverInfo, geckodriver_info::GeckodriverInfo,
    };
    pub use crate::os_specific;
    pub use crate::traits::installation_info::{
        AddExecutePermissionError, InstallationError, WebdriverInstallationInfo,
    };
    pub use crate::traits::url_info::{UrlError, WebdriverUrlInfo, WebdriverVersionUrl};
    pub use crate::traits::verification_info::{VerificationError, WebdriverVerificationInfo};
    pub use crate::traits::webdriver_download_info::{
        WebdriverDownloadError, WebdriverDownloadInfo,
    };
}
