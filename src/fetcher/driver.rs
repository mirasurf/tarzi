//! Web Driver Manager for browser automation
//!
//! This module provides a comprehensive web driver management system that supports
//! multiple browser drivers (chromedriver, geckodriver, etc.) with lifecycle management,
//! status checking, and automatic cleanup.

use crate::{
    Result, TarziError,
    constants::{CHROMEDRIVER_DEFAULT_PORT, DEFAULT_TIMEOUT_SECS},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

/// Supported web driver types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DriverType {
    /// ChromeDriver for Chrome and Chromium browsers
    Chrome,
    /// GeckoDriver for Firefox browser
    Firefox,
    /// Generic driver type for future extensions
    Generic(String),
}

impl std::fmt::Display for DriverType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DriverType::Chrome => write!(f, "chromedriver"),
            DriverType::Firefox => write!(f, "geckodriver"),
            DriverType::Generic(name) => write!(f, "{name}"),
        }
    }
}

impl std::str::FromStr for DriverType {
    type Err = TarziError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "chromedriver" | "chrome" => Ok(DriverType::Chrome),
            "geckodriver" | "firefox" => Ok(DriverType::Firefox),
            _ => Ok(DriverType::Generic(s.to_string())),
        }
    }
}

/// Configuration for a web driver
#[derive(Debug, Clone)]
pub struct DriverConfig {
    /// Type of driver
    pub driver_type: DriverType,
    /// Port to run the driver on
    pub port: u16,
    /// Additional command line arguments
    pub args: Vec<String>,
    /// Timeout for driver operations (in seconds)
    pub timeout: Duration,
    /// Whether to enable verbose logging
    pub verbose: bool,
}

impl Default for DriverConfig {
    fn default() -> Self {
        Self {
            driver_type: DriverType::Chrome,
            port: CHROMEDRIVER_DEFAULT_PORT,
            args: Vec::new(),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            verbose: false,
        }
    }
}

/// Status of a web driver process
#[derive(Debug, Clone, PartialEq)]
pub enum DriverStatus {
    /// Driver is not running
    Stopped,
    /// Driver is starting up
    Starting,
    /// Driver is running and ready
    Running,
    /// Driver has failed
    Failed(String),
}

/// Information about a running driver
#[derive(Debug, Clone)]
pub struct DriverInfo {
    /// Configuration used to start the driver
    pub config: DriverConfig,
    /// Current status of the driver
    pub status: DriverStatus,
    /// Process ID of the driver
    pub pid: Option<u32>,
    /// Time when the driver was started
    pub started_at: Instant,
    /// WebDriver endpoint URL
    pub endpoint: String,
}

/// A running web driver process
#[derive(Debug)]
struct DriverProcess {
    /// The child process
    child: Child,
    /// Configuration
    config: DriverConfig,
    /// Start time
    started_at: Instant,
}

/// Web Driver Manager
///
/// Manages the lifecycle of web driver processes, supporting multiple driver types
/// and providing status monitoring, health checks, and automatic cleanup.
#[derive(Debug)]
pub struct DriverManager {
    /// Map of running drivers by port
    drivers: Arc<Mutex<HashMap<u16, DriverProcess>>>,
    /// Default configuration
    default_config: DriverConfig,
}

impl DriverManager {
    /// Create a new driver manager with default configuration
    pub fn new() -> Self {
        Self {
            drivers: Arc::new(Mutex::new(HashMap::new())),
            default_config: DriverConfig::default(),
        }
    }

    /// Create a new driver manager with custom default configuration
    pub fn with_config(config: DriverConfig) -> Self {
        Self {
            drivers: Arc::new(Mutex::new(HashMap::new())),
            default_config: config,
        }
    }

    /// Start a web driver with default configuration
    pub fn start_driver(&self) -> Result<DriverInfo> {
        self.start_driver_with_config(self.default_config.clone())
    }

    /// Start a web driver with custom configuration
    pub fn start_driver_with_config(&self, config: DriverConfig) -> Result<DriverInfo> {
        // Check if driver binary exists
        self.check_driver_binary(&config.driver_type)?;

        // Check if port is already in use
        if self.is_port_in_use(config.port) {
            return Err(TarziError::Driver(format!(
                "Port {} is already in use",
                config.port
            )));
        }

        // Build command
        let mut cmd = Command::new(self.get_driver_binary_name(&config.driver_type));
        cmd.arg(format!("--port={}", config.port));

        // Add driver-specific arguments
        match config.driver_type {
            DriverType::Chrome => {
                cmd.arg("--whitelisted-ips=");
                if config.verbose {
                    cmd.arg("--verbose");
                }
            }
            DriverType::Firefox => {
                cmd.arg("--host=127.0.0.1");
                if config.verbose {
                    cmd.args(["--log", "debug"]);
                }
            }
            DriverType::Generic(_) => {
                // Generic drivers may not support standard arguments
            }
        }

        // Add custom arguments
        for arg in &config.args {
            cmd.arg(arg);
        }

        // Set up process stdio
        cmd.stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::null());

        // Start the process
        let child = cmd.spawn().map_err(|e| {
            TarziError::DriverProcess(format!(
                "Failed to start {} driver: {}",
                config.driver_type, e
            ))
        })?;

        let pid = child.id();
        let started_at = Instant::now();

        // Store the driver process
        let driver_process = DriverProcess {
            child,
            config: config.clone(),
            started_at,
        };

        {
            let mut drivers = self.drivers.lock().unwrap();
            drivers.insert(config.port, driver_process);
        }

        // Wait for driver to be ready
        let endpoint = format!("http://127.0.0.1:{}", config.port);
        self.wait_for_driver_ready(&endpoint, config.timeout)?;

        Ok(DriverInfo {
            config,
            status: DriverStatus::Running,
            pid: Some(pid),
            started_at,
            endpoint,
        })
    }

    /// Stop a driver by port
    pub fn stop_driver(&self, port: u16) -> Result<()> {
        let mut drivers = self.drivers.lock().unwrap();

        if let Some(mut driver_process) = drivers.remove(&port) {
            // Try to terminate gracefully first
            if let Err(e) = driver_process.child.kill() {
                log::warn!("Failed to kill driver process: {e}");
            }

            // Wait for process to exit
            if let Err(e) = driver_process.child.wait() {
                log::warn!("Failed to wait for driver process to exit: {e}");
            }

            log::info!(
                "Stopped {} driver on port {}",
                driver_process.config.driver_type,
                port
            );
            Ok(())
        } else {
            Err(TarziError::Driver(format!(
                "No driver running on port {port}"
            )))
        }
    }

    /// Stop all running drivers
    pub fn stop_all_drivers(&self) -> Result<()> {
        let ports: Vec<u16> = {
            let drivers = self.drivers.lock().unwrap();
            drivers.keys().cloned().collect()
        };

        for port in ports {
            if let Err(e) = self.stop_driver(port) {
                log::warn!("Failed to stop driver on port {port}: {e}");
            }
        }

        Ok(())
    }

    /// Get information about a running driver
    pub fn get_driver_info(&self, port: u16) -> Option<DriverInfo> {
        let drivers = self.drivers.lock().unwrap();

        drivers.get(&port).map(|driver_process| {
            let status = if self.is_driver_healthy(&format!("http://127.0.0.1:{port}")) {
                DriverStatus::Running
            } else {
                DriverStatus::Failed("Driver not responding".to_string())
            };

            DriverInfo {
                config: driver_process.config.clone(),
                status,
                pid: Some(driver_process.child.id()),
                started_at: driver_process.started_at,
                endpoint: format!("http://127.0.0.1:{port}"),
            }
        })
    }

    /// List all running drivers
    pub fn list_drivers(&self) -> Vec<DriverInfo> {
        let drivers = self.drivers.lock().unwrap();

        drivers
            .iter()
            .map(|(port, driver_process)| {
                let status = if self.is_driver_healthy(&format!("http://127.0.0.1:{}", *port)) {
                    DriverStatus::Running
                } else {
                    DriverStatus::Failed("Driver not responding".to_string())
                };

                DriverInfo {
                    config: driver_process.config.clone(),
                    status,
                    pid: Some(driver_process.child.id()),
                    started_at: driver_process.started_at,
                    endpoint: format!("http://127.0.0.1:{port}"),
                }
            })
            .collect()
    }

    /// Check if a driver binary is installed
    pub fn check_driver_binary(&self, driver_type: &DriverType) -> Result<()> {
        let binary_name = self.get_driver_binary_name(driver_type);

        // Try to find the binary in PATH
        match which::which(&binary_name) {
            Ok(path) => {
                log::debug!("Found {binary_name} at {path:?}");
                Ok(())
            }
            Err(_) => {
                let install_message = match driver_type {
                    DriverType::Chrome => {
                        "Please install ChromeDriver: https://chromedriver.chromium.org/"
                    }
                    DriverType::Firefox => {
                        "Please install GeckoDriver: https://github.com/mozilla/geckodriver/releases"
                    }
                    DriverType::Generic(name) => {
                        return Err(TarziError::DriverNotFound(format!(
                            "Driver '{name}' not found in PATH. Please ensure it's installed and available."
                        )));
                    }
                };

                Err(TarziError::DriverNotFound(format!(
                    "{binary_name} not found in PATH. {install_message}"
                )))
            }
        }
    }

    /// Check if a port is in use by this manager
    pub fn is_port_in_use(&self, port: u16) -> bool {
        let drivers = self.drivers.lock().unwrap();
        drivers.contains_key(&port)
    }

    /// Perform a health check on a driver
    pub fn is_driver_healthy(&self, endpoint: &str) -> bool {
        // Use a simple TCP connection check instead of HTTP to avoid blocking runtime issues
        use std::net::TcpStream;

        if let Ok(stream) = TcpStream::connect_timeout(
            &endpoint.replace("http://", "").parse().unwrap(),
            Duration::from_secs(2),
        ) {
            stream.shutdown(std::net::Shutdown::Both).ok();
            true
        } else {
            false
        }
    }

    /// Wait for a driver to be ready
    fn wait_for_driver_ready(&self, endpoint: &str, timeout: Duration) -> Result<()> {
        let start = Instant::now();

        while start.elapsed() < timeout {
            if self.is_driver_healthy(endpoint) {
                return Ok(());
            }

            thread::sleep(Duration::from_millis(500));
        }

        Err(TarziError::Driver(format!(
            "Driver failed to become ready within {:?}",
            timeout
        )))
    }

    /// Get the binary name for a driver type
    fn get_driver_binary_name(&self, driver_type: &DriverType) -> String {
        match driver_type {
            DriverType::Chrome => "chromedriver".to_string(),
            DriverType::Firefox => "geckodriver".to_string(),
            DriverType::Generic(name) => name.clone(),
        }
    }

    /// Get supported driver types
    pub fn supported_drivers() -> Vec<DriverType> {
        vec![DriverType::Chrome, DriverType::Firefox]
    }

    /// Create a driver config for a specific type
    pub fn create_config(driver_type: DriverType, port: u16) -> DriverConfig {
        DriverConfig {
            driver_type,
            port,
            args: Vec::new(),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            verbose: false,
        }
    }
}

impl Default for DriverManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for DriverManager {
    fn drop(&mut self) {
        // Clean up all running drivers when the manager is dropped
        // Use a simple approach that doesn't block the async runtime
        if let Ok(mut drivers) = self.drivers.lock() {
            for (port, mut driver_process) in drivers.drain() {
                let _ = driver_process.child.kill();
                log::info!("Killed driver process on port {}", port);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_type_from_str() {
        assert_eq!(
            "chromedriver".parse::<DriverType>().unwrap(),
            DriverType::Chrome
        );
        assert_eq!("chrome".parse::<DriverType>().unwrap(), DriverType::Chrome);
        assert_eq!(
            "geckodriver".parse::<DriverType>().unwrap(),
            DriverType::Firefox
        );
        assert_eq!(
            "firefox".parse::<DriverType>().unwrap(),
            DriverType::Firefox
        );

        match "custom".parse::<DriverType>().unwrap() {
            DriverType::Generic(name) => assert_eq!(name, "custom"),
            _ => panic!("Expected Generic driver type"),
        }
    }

    #[test]
    fn test_driver_type_display() {
        assert_eq!(DriverType::Chrome.to_string(), "chromedriver");
        assert_eq!(DriverType::Firefox.to_string(), "geckodriver");
        assert_eq!(
            DriverType::Generic("custom".to_string()).to_string(),
            "custom"
        );
    }

    #[test]
    fn test_driver_config_default() {
        let config = DriverConfig::default();
        assert_eq!(config.driver_type, DriverType::Chrome);
        assert_eq!(config.port, CHROMEDRIVER_DEFAULT_PORT);
        assert_eq!(config.timeout, Duration::from_secs(DEFAULT_TIMEOUT_SECS));
        assert!(!config.verbose);
        assert!(config.args.is_empty());
    }

    #[test]
    fn test_driver_manager_new() {
        let manager = DriverManager::new();
        assert_eq!(manager.default_config.driver_type, DriverType::Chrome);
        assert_eq!(manager.default_config.port, CHROMEDRIVER_DEFAULT_PORT);
    }

    #[test]
    fn test_driver_manager_with_config() {
        let config = DriverConfig {
            driver_type: DriverType::Firefox,
            port: 19515, // Use a different port for testing
            args: vec!["--verbose".to_string()],
            timeout: Duration::from_secs(10),
            verbose: true,
        };

        let manager = DriverManager::with_config(config.clone());
        assert_eq!(manager.default_config.driver_type, config.driver_type);
        assert_eq!(manager.default_config.port, config.port);
        assert_eq!(manager.default_config.args, config.args);
        assert_eq!(manager.default_config.timeout, config.timeout);
        assert_eq!(manager.default_config.verbose, config.verbose);
    }

    #[test]
    fn test_supported_drivers() {
        let drivers = DriverManager::supported_drivers();
        assert!(drivers.contains(&DriverType::Chrome));
        assert!(drivers.contains(&DriverType::Firefox));
        assert_eq!(drivers.len(), 2);
    }

    #[test]
    fn test_create_config() {
        let config = DriverManager::create_config(DriverType::Firefox, 19515);
        assert_eq!(config.driver_type, DriverType::Firefox);
        assert_eq!(config.port, 19515);
        assert_eq!(config.timeout, Duration::from_secs(DEFAULT_TIMEOUT_SECS));
        assert!(!config.verbose);
    }

    #[test]
    fn test_is_port_in_use() {
        let manager = DriverManager::new();
        assert!(!manager.is_port_in_use(CHROMEDRIVER_DEFAULT_PORT));
        assert!(!manager.is_port_in_use(19515));
    }

    #[test]
    fn test_driver_binary_name() {
        let manager = DriverManager::new();
        assert_eq!(
            manager.get_driver_binary_name(&DriverType::Chrome),
            "chromedriver"
        );
        assert_eq!(
            manager.get_driver_binary_name(&DriverType::Firefox),
            "geckodriver"
        );
        assert_eq!(
            manager.get_driver_binary_name(&DriverType::Generic("custom".to_string())),
            "custom"
        );
    }

    #[test]
    fn test_list_drivers_empty() {
        let manager = DriverManager::new();
        let drivers = manager.list_drivers();
        assert!(drivers.is_empty());
    }

    #[test]
    fn test_get_driver_info_not_found() {
        let manager = DriverManager::new();
        let info = manager.get_driver_info(CHROMEDRIVER_DEFAULT_PORT);
        assert!(info.is_none());
    }

    #[test]
    fn test_stop_driver_not_found() {
        let manager = DriverManager::new();
        let result = manager.stop_driver(CHROMEDRIVER_DEFAULT_PORT);
        assert!(result.is_err());

        if let Err(TarziError::Driver(msg)) = result {
            assert!(msg.contains(&format!(
                "No driver running on port {CHROMEDRIVER_DEFAULT_PORT}"
            )));
        } else {
            panic!("Expected Driver error");
        }
    }

    #[test]
    fn test_driver_status_equality() {
        assert_eq!(DriverStatus::Stopped, DriverStatus::Stopped);
        assert_eq!(DriverStatus::Starting, DriverStatus::Starting);
        assert_eq!(DriverStatus::Running, DriverStatus::Running);
        assert_eq!(
            DriverStatus::Failed("test".to_string()),
            DriverStatus::Failed("test".to_string())
        );

        assert_ne!(DriverStatus::Stopped, DriverStatus::Running);
        assert_ne!(
            DriverStatus::Failed("test1".to_string()),
            DriverStatus::Failed("test2".to_string())
        );
    }

    // Note: Integration tests that actually start driver processes are in the tests/ directory
    // These unit tests focus on the logic and structure without requiring actual driver binaries
}

// Integration test helper functions
#[cfg(any(test, feature = "test-helpers"))]
pub mod test_helpers {
    use super::*;

    /// Check if a driver binary is available for testing
    pub fn is_driver_available(driver_type: &DriverType) -> bool {
        let manager = DriverManager::new();
        manager.check_driver_binary(driver_type).is_ok()
    }

    /// Create a test driver manager with specific configuration
    pub fn create_test_manager() -> DriverManager {
        let config = DriverConfig {
            driver_type: DriverType::Chrome,
            port: 19515, // Use a different port for testing
            args: vec!["--disable-gpu".to_string(), "--no-sandbox".to_string()],
            timeout: Duration::from_secs(10),
            verbose: true,
        };
        DriverManager::with_config(config)
    }

    /// Find an available port for testing
    pub fn find_available_port() -> u16 {
        use std::net::TcpListener;

        for port in 19515..19600 {
            if TcpListener::bind(("127.0.0.1", port)).is_ok() {
                return port;
            }
        }

        panic!("No available ports found for testing");
    }
}
