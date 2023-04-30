use std::ffi::OsStr;
use std::path::Path;
use std::process::Child;
use std::thread;

use async_trait::async_trait;
use fantoccini::wd::Capabilities;
use fantoccini::Locator;

struct ChildGuard(pub Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        if let Err(e) = self.0.kill() {
            println!("Failed to kill child process: {}", e);
        }
    }
}

/// Error that can occur during verification.
#[derive(thiserror::Error, Debug)]
pub enum VerificationError {
    #[error("Failed to start driver: {0}")]
    Start(#[from] std::io::Error),
    #[error("Failed to connect to driver: {0}")]
    Connect(#[from] fantoccini::error::NewSessionError),
    #[error("Driver test failed to pass: {0}")]
    Navigate(#[from] fantoccini::error::CmdError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Provides information for verifying a webdriver.
#[async_trait]
pub trait WebdriverVerificationInfo {
    /// Capabilities to use for verification.
    /// Some driver options such as browser path can be provided by capabilities.
    fn driver_capabilities(&self) -> Option<Capabilities>;

    /// Verifies driver using [test_client](WebdriverVerificationInfo::test_client).
    async fn verify_driver<P: AsRef<Path> + Sync>(
        &self,
        driver_path: &P,
    ) -> Result<(), VerificationError> {
        let port = get_random_available_port();
        let _child = ChildGuard(
            std::process::Command::new(OsStr::new(driver_path.as_ref()))
                .arg(&format!("--port={}", port))
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()?,
        );

        thread::sleep(std::time::Duration::from_millis(500));

        let client;
        if let Some(capabilities) = self.driver_capabilities() {
            client = fantoccini::ClientBuilder::native()
                .capabilities(capabilities)
                .connect(&format!("http://localhost:{}", port))
                .await?;
        } else {
            client = fantoccini::ClientBuilder::native()
                .connect(&format!("http://localhost:{}", port))
                .await?;
        }

        let test_result = Self::test_client(&client).await;

        if let Err(e) = client.close().await {
            println!("Failed to close client: {}", e);
        }

        test_result
    }

    async fn test_client(client: &fantoccini::Client) -> Result<(), VerificationError> {
        client.goto("https://www.example.com").await?;
        client.find(Locator::Css("html")).await?;

        Ok(())
    }
}

fn get_random_available_port() -> u16 {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0);
    let listener = TcpListener::bind(addr).unwrap();
    listener.local_addr().unwrap().port()
}

#[cfg(test)]
mod tests {
    use std::net::TcpListener;

    use super::get_random_available_port;

    #[test]
    fn test_get_random_available_port() {
        let port = get_random_available_port();

        // Check if the port number is within the valid range
        assert!(port > 0, "Port number should be within the valid range");

        // Check if the port is actually available
        let addr = format!("127.0.0.1:{}", port);
        let listener_result = TcpListener::bind(addr);
        assert!(
            listener_result.is_ok(),
            "Port number should be available for binding"
        );
    }
}
