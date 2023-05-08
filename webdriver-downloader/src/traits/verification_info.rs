use std::ffi::OsStr;
use std::io::BufReader;
use std::path::Path;
use std::process::Child;

use async_trait::async_trait;
use fantoccini::wd::Capabilities;
use fantoccini::Locator;
use std::io::prelude::*;
use std::thread;

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
    #[error("Webdriver failed to initialize: {0}")]
    WebdriverInit(String),
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

    /// String that is printed by webdriver when it is started successfully.
    fn driver_started_stdout_string() -> &'static str;

    /// Verifies driver using [test_client](WebdriverVerificationInfo::test_client).
    async fn verify_driver<P: AsRef<Path> + Sync>(
        &self,
        driver_path: &P,
    ) -> Result<(), VerificationError> {
        let port = get_random_available_port();
        let _child_guard = ChildGuard(
            wait_for_driver_start(driver_path, port, Self::driver_started_stdout_string()).await?,
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

async fn wait_for_driver_start<P: AsRef<Path> + Sync>(
    driver_path: &P,
    port: u16,
    initialize_str: &str,
) -> Result<Child, VerificationError> {
    let mut child = std::process::Command::new(OsStr::new(driver_path.as_ref()))
        .arg(&format!("--port={}", port))
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let reader = BufReader::new(stdout);
    for line in reader.lines().flatten() {
        if line.contains(initialize_str) {
            return Ok(child);
        }
    }

    let reader = BufReader::new(stderr);
    let err_string = reader
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()
        .join("\n");

    Err(VerificationError::WebdriverInit(err_string))
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
