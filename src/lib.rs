pub use driver_management::{
    WebdriverInstallationInfo, WebdriverUrlInfo, WebdriverVerificationInfo,
};

pub mod cli;

pub mod driver_management;

pub trait WebdriverInfo:
    WebdriverUrlInfo + WebdriverInstallationInfo + WebdriverVerificationInfo + Sync
{
}

impl<T> WebdriverInfo for T where
    T: WebdriverUrlInfo + WebdriverInstallationInfo + WebdriverVerificationInfo + Sync
{
}
