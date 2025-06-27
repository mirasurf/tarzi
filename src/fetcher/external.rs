use crate::{Result, error::TarziError};
use std::time::Duration;
use tempfile::TempDir;
use thirtyfour::{DesiredCapabilities, WebDriver};
use tracing::{error, info, warn};

/// External browser connection manager
pub struct ExternalBrowserManager {
    browsers: std::collections::HashMap<String, (WebDriver, TempDir)>,
}

impl ExternalBrowserManager {
    pub fn new() -> Self {
        Self {
            browsers: std::collections::HashMap::new(),
        }
    }

    /// Connect to an external browser instance
    pub async fn connect_to_external_browser(&mut self, ws_endpoint: &str) -> Result<()> {
        info!(
            "Attempting to connect to external browser at: {}",
            ws_endpoint
        );

        // Check if the endpoint is accessible
        if !self
            .check_external_browser_prerequisites(ws_endpoint)
            .await?
        {
            return Err(TarziError::Browser(
                "External browser prerequisites not met".to_string(),
            ));
        }

        info!("Prerequisites met, connecting to external browser...");

        // For thirtyfour, we connect to a WebDriver server
        // Convert WebSocket endpoint to HTTP endpoint if needed
        let webdriver_url = if ws_endpoint.starts_with("ws://") {
            ws_endpoint.replace("ws://", "http://").replace("/ws", "")
        } else if ws_endpoint.starts_with("wss://") {
            ws_endpoint.replace("wss://", "https://").replace("/ws", "")
        } else {
            ws_endpoint.to_string()
        };

        let caps = DesiredCapabilities::chrome();
        let browser_result = tokio::time::timeout(
            Duration::from_secs(30), // 30 seconds for connection
            WebDriver::new(&webdriver_url, caps),
        )
        .await;

        let browser = match browser_result {
            Ok(Ok(result)) => {
                info!("Successfully connected to external browser");
                result
            }
            Ok(Err(e)) => {
                error!("Failed to connect to external browser: {}", e);
                return Err(TarziError::Browser(format!(
                    "Failed to connect to external browser: {}",
                    e
                )));
            }
            Err(_) => {
                error!("Timeout while connecting to external browser (30 seconds)");
                return Err(TarziError::Browser(
                    "Timeout while connecting to external browser".to_string(),
                ));
            }
        };

        let temp_dir = TempDir::new()?;
        self.browsers
            .insert("external".to_string(), (browser, temp_dir));
        info!("External browser connection established and stored");
        Ok(())
    }

    /// Check prerequisites for external browser connection
    pub async fn check_external_browser_prerequisites(&self, ws_endpoint: &str) -> Result<bool> {
        info!(
            "Checking external browser prerequisites for endpoint: {}",
            ws_endpoint
        );

        // Check if the endpoint URL is valid
        if !ws_endpoint.starts_with("ws://") && !ws_endpoint.starts_with("wss://") {
            warn!("Invalid WebSocket endpoint format: {}", ws_endpoint);
            return Ok(false);
        }

        // FIXME (2025-06-26): For now, we'll assume the endpoint is valid if it has the correct format
        info!("Basic WebSocket endpoint format validation passed");

        // FIXME (2025-06-26): Try to establish a basic WebSocket connection to check if the browser is accessible
        // info!("Attempting basic WebSocket connectivity check...");

        Ok(true)
    }

    /// Get the external browser instance
    pub fn get_external_browser(&self) -> Option<&WebDriver> {
        self.browsers.get("external").map(|(browser, _)| browser)
    }

    /// Check if external browser is connected
    pub fn is_connected(&self) -> bool {
        self.browsers.contains_key("external")
    }

    /// Get default external browser endpoint
    pub fn get_default_endpoint() -> String {
        std::env::var("TARZI_EXTERNAL_BROWSER_ENDPOINT")
            .unwrap_or_else(|_| "ws://localhost:9222".to_string())
    }
}

impl Default for ExternalBrowserManager {
    fn default() -> Self {
        Self::new()
    }
}
