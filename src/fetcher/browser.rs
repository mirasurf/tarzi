use super::driver::{DriverConfig, DriverInfo, DriverManager, DriverType};
use crate::{
    Result,
    config::Config,
    constants::{
        BROWSER_LAUNCH_TIMEOUT, CHROME_DRIVER_ARGS, CHROMEDRIVER_DEFAULT_PORT,
        CHROMEDRIVER_DEFAULT_URL, DEFAULT_TIMEOUT, FIREFOX_DRIVER_ARGS, GECKODRIVER_DEFAULT_PORT,
        WEBDRIVER_CHECK_TIMEOUT,
    },
    error::TarziError,
    utils::is_webdriver_available,
};
use std::{collections::HashMap, path::PathBuf};
use tempfile::TempDir;
use thirtyfour::{ChromiumLikeCapabilities, DesiredCapabilities, WebDriver};
use tracing::{error, info, warn};

/// Browser instance manager
pub struct BrowserManager {
    browsers: HashMap<String, (WebDriver, TempDir)>,
    driver_manager: Option<DriverManager>,
    managed_driver_info: Option<DriverInfo>,
}

impl BrowserManager {
    pub fn new() -> Self {
        Self {
            browsers: HashMap::new(),
            driver_manager: None,
            managed_driver_info: None,
        }
    }

    /// Create a new BrowserManager with configuration
    pub fn from_config(_config: &Config) -> Self {
        Self {
            browsers: HashMap::new(),
            driver_manager: None,
            managed_driver_info: None,
        }
    }

    /// Create a new browser instance with a specific user data directory
    pub async fn create_browser_with_user_data(
        &mut self,
        user_data_dir: Option<PathBuf>,
        headless: bool,
        instance_id: Option<String>,
    ) -> Result<String> {
        let webdriver_url = self.get_or_create_webdriver_endpoint().await?;

        let instance_id = instance_id.unwrap_or_else(|| {
            let temp_dir = TempDir::new().expect("Failed to create temp dir for instance ID");
            temp_dir.path().to_string_lossy().to_string()
        });
        info!(
            "Creating new browser instance with ID: {} (headless: {}, user_data_dir: {:?})",
            instance_id, headless, user_data_dir
        );

        // For thirtyfour, we need a WebDriver server running
        // We'll use the default ChromeDriver endpoint

        let mut caps = DesiredCapabilities::chrome();
        if headless {
            caps.add_arg("--headless").map_err(|e| {
                error!("Failed to add headless arg: {}", e);
                TarziError::Browser(format!("Failed to add headless arg: {}", e))
            })?;
        }
        caps.add_arg("--disable-gpu").map_err(|e| {
            error!("Failed to add disable-gpu arg: {}", e);
            TarziError::Browser(format!("Failed to add disable-gpu arg: {}", e))
        })?;
        caps.add_arg("--disable-dev-shm-usage").map_err(|e| {
            error!("Failed to add disable-dev-shm-usage arg: {}", e);
            TarziError::Browser(format!("Failed to add disable-dev-shm-usage arg: {}", e))
        })?;
        caps.add_arg("--no-sandbox").map_err(|e| {
            error!("Failed to add no-sandbox arg: {}", e);
            TarziError::Browser(format!("Failed to add no-sandbox arg: {}", e))
        })?;

        let temp_dir = if let Some(user_data_path) = user_data_dir {
            let user_data_arg = format!("--user-data-dir={}", user_data_path.to_string_lossy());
            caps.add_arg(&user_data_arg).map_err(|e| {
                error!("Failed to add user-data-dir arg: {}", e);
                TarziError::Browser(format!("Failed to add user-data-dir arg: {}", e))
            })?;
            None
        } else {
            let temp = TempDir::new().map_err(|e| {
                error!("Failed to create temporary directory: {}", e);
                TarziError::Browser(format!("Failed to create temporary directory: {}", e))
            })?;
            let user_data_arg = format!("--user-data-dir={}", temp.path().to_string_lossy());
            caps.add_arg(&user_data_arg).map_err(|e| {
                error!("Failed to add user-data-dir arg: {}", e);
                TarziError::Browser(format!("Failed to add user-data-dir arg: {}", e))
            })?;
            Some(temp)
        };

        info!("Browser config created successfully");
        let browser_result =
            tokio::time::timeout(BROWSER_LAUNCH_TIMEOUT, WebDriver::new(&webdriver_url, caps))
                .await;
        let browser = match browser_result {
            Ok(Ok(result)) => {
                info!("Browser launched successfully with ID: {}", instance_id);
                result
            }
            Ok(Err(e)) => {
                error!("Failed to create browser: {}", e);
                return Err(TarziError::Browser(format!(
                    "Failed to create browser: {}",
                    e
                )));
            }
            Err(_) => {
                error!("Timeout while launching browser (60 seconds)");
                return Err(TarziError::Browser(
                    "Timeout while launching browser".to_string(),
                ));
            }
        };
        self.browsers.insert(
            instance_id.clone(),
            (
                browser,
                temp_dir.unwrap_or_else(|| {
                    TempDir::new().expect("Failed to create temp dir for browser storage")
                }),
            ),
        );
        info!("Browser instance stored with ID: {}", instance_id);
        Ok(instance_id)
    }

    /// Get a browser instance by ID
    pub fn get_browser(&self, instance_id: &str) -> Option<&WebDriver> {
        self.browsers.get(instance_id).map(|(browser, _)| browser)
    }

    /// Get all browser instance IDs
    pub fn get_browser_ids(&self) -> Vec<String> {
        self.browsers.keys().cloned().collect()
    }

    /// Remove a browser instance by ID
    pub async fn remove_browser(&mut self, instance_id: &str) -> Result<bool> {
        if let Some((driver, _temp_dir)) = self.browsers.remove(instance_id) {
            info!("Removed browser instance: {}", instance_id);
            driver.quit().await.map_err(|e| {
                error!("Failed to quit browser: {}", e);
                TarziError::Browser(format!("Failed to quit browser: {}", e))
            })?;
            // The temp_dir will be automatically cleaned up when dropped
            Ok(true)
        } else {
            warn!("Browser instance not found: {}", instance_id);
            Ok(false)
        }
    }

    /// Get or create a browser instance
    pub async fn get_or_create_browser(&mut self, headless: bool) -> Result<&WebDriver> {
        if self.browsers.is_empty() {
            info!("Creating new browser instance (headless: {})...", headless);
            let instance_id = self
                .create_browser_with_user_data(None, headless, Some("default".to_string()))
                .await?;
            info!("Browser instance created with ID: {}", instance_id);
        } else {
            info!("Using existing browser instance");
        }
        Ok(&self.browsers.values().next().unwrap().0)
    }

    /// Create multiple browser instances for parallel processing
    pub async fn create_multiple_browsers(
        &mut self,
        count: usize,
        headless: bool,
        base_instance_id: Option<String>,
    ) -> Result<Vec<String>> {
        info!(
            "Creating {} browser instances (headless: {})",
            count, headless
        );

        let base_id = base_instance_id.unwrap_or_else(|| "browser".to_string());
        let mut instance_ids = Vec::new();

        for i in 0..count {
            let instance_id = format!("{}_{}", base_id, i);
            let id = self
                .create_browser_with_user_data(None, headless, Some(instance_id.clone()))
                .await?;
            instance_ids.push(id);
        }

        info!("Successfully created {} browser instances", count);
        Ok(instance_ids)
    }

    /// Check if any browsers are available
    pub fn has_browsers(&self) -> bool {
        !self.browsers.is_empty()
    }

    /// Get the first available browser
    pub fn get_first_browser(&self) -> Option<&WebDriver> {
        self.browsers.values().next().map(|(browser, _)| browser)
    }

    /// Get or create a webdriver endpoint, prioritizing TARZI_WEBDRIVER_URL
    /// If TARZI_WEBDRIVER_URL is not set and no webdriver is available, use DriverManager
    async fn get_or_create_webdriver_endpoint(&mut self) -> Result<String> {
        // First, check if TARZI_WEBDRIVER_URL is set
        if let Ok(webdriver_url) = std::env::var("TARZI_WEBDRIVER_URL") {
            if !webdriver_url.is_empty() {
                info!("Using TARZI_WEBDRIVER_URL: {}", webdriver_url);

                // Check if the specified webdriver is available
                info!("Checking WebDriver availability at: {}", webdriver_url);
                if is_webdriver_available().await {
                    info!("WebDriver server is available and ready");
                    return Ok(webdriver_url);
                } else {
                    warn!(
                        "TARZI_WEBDRIVER_URL is set but WebDriver is not available at: {}",
                        webdriver_url
                    );
                    return Err(TarziError::Browser(format!(
                        "WebDriver server is not available at {}. Please start the WebDriver server.",
                        webdriver_url
                    )));
                }
            }
        }

        // If TARZI_WEBDRIVER_URL is not set or empty, try default WebDriver URL
        let default_url = CHROMEDRIVER_DEFAULT_URL.to_string();
        info!(
            "TARZI_WEBDRIVER_URL not set, checking default WebDriver at: {}",
            default_url
        );

        if is_webdriver_available_at_url(&default_url).await {
            info!("WebDriver server found at default URL: {}", default_url);
            return Ok(default_url);
        }

        // If no webdriver is available, try to start one using DriverManager
        info!("No WebDriver server found, attempting to start one using DriverManager");

        // Initialize DriverManager if not already done
        if self.driver_manager.is_none() {
            info!("Initializing DriverManager");
            self.driver_manager = Some(DriverManager::new());
        }

        // Try to start a driver using DriverManager
        let driver_manager = self.driver_manager.as_ref().unwrap();

        // First check if chromedriver is available
        match driver_manager.check_driver_binary(&DriverType::Chrome) {
            Ok(()) => {
                info!("ChromeDriver found, starting driver with DriverManager");
                let config = DriverConfig {
                    driver_type: DriverType::Chrome,
                    port: CHROMEDRIVER_DEFAULT_PORT,
                    args: CHROME_DRIVER_ARGS.iter().map(|s| s.to_string()).collect(),
                    timeout: DEFAULT_TIMEOUT,
                    verbose: false,
                };

                match driver_manager.start_driver_with_config(config) {
                    Ok(driver_info) => {
                        info!(
                            "Successfully started ChromeDriver at: {}",
                            driver_info.endpoint
                        );
                        self.managed_driver_info = Some(driver_info.clone());
                        return Ok(driver_info.endpoint);
                    }
                    Err(e) => {
                        warn!("Failed to start ChromeDriver with DriverManager: {}", e);
                    }
                }
            }
            Err(e) => {
                warn!("ChromeDriver not available: {}", e);
            }
        }

        // Try Firefox as fallback
        match driver_manager.check_driver_binary(&DriverType::Firefox) {
            Ok(()) => {
                info!("GeckoDriver found, starting driver with DriverManager");
                let config = DriverConfig {
                    driver_type: DriverType::Firefox,
                    port: GECKODRIVER_DEFAULT_PORT,
                    args: FIREFOX_DRIVER_ARGS.iter().map(|s| s.to_string()).collect(),
                    timeout: DEFAULT_TIMEOUT,
                    verbose: false,
                };

                match driver_manager.start_driver_with_config(config) {
                    Ok(driver_info) => {
                        info!(
                            "Successfully started GeckoDriver at: {}",
                            driver_info.endpoint
                        );
                        self.managed_driver_info = Some(driver_info.clone());
                        return Ok(driver_info.endpoint);
                    }
                    Err(e) => {
                        warn!("Failed to start GeckoDriver with DriverManager: {}", e);
                    }
                }
            }
            Err(e) => {
                warn!("GeckoDriver not available: {}", e);
            }
        }

        // If all attempts failed, return an error with helpful guidance
        Err(TarziError::Browser(
            "No WebDriver server is available. Please either:\n\
            1. Set TARZI_WEBDRIVER_URL environment variable to your WebDriver endpoint, or\n\
            2. Install ChromeDriver (https://chromedriver.chromium.org/) or GeckoDriver (https://github.com/mozilla/geckodriver/releases) and ensure they're in your PATH, or\n\
            3. Start a WebDriver server manually and set TARZI_WEBDRIVER_URL".to_string()
        ))
    }

    /// Clean up managed driver if any
    pub async fn cleanup_managed_driver(&mut self) -> Result<()> {
        if let (Some(driver_manager), Some(driver_info)) =
            (&mut self.driver_manager, &self.managed_driver_info)
        {
            info!("Cleaning up managed driver: {}", driver_info.endpoint);
            match driver_manager.stop_driver(driver_info.config.port) {
                Ok(()) => {
                    info!("Successfully stopped managed driver");
                    self.managed_driver_info = None;
                }
                Err(e) => {
                    warn!("Failed to stop managed driver: {}", e);
                    return Err(TarziError::Browser(format!(
                        "Failed to stop managed driver: {}",
                        e
                    )));
                }
            }
        }
        Ok(())
    }

    /// Check if this browser manager has a managed driver
    pub fn has_managed_driver(&self) -> bool {
        self.managed_driver_info.is_some()
    }

    /// Get information about the managed driver
    pub fn get_managed_driver_info(&self) -> Option<&DriverInfo> {
        self.managed_driver_info.as_ref()
    }
}

/// Helper function to check if webdriver is available at a specific URL
async fn is_webdriver_available_at_url(url: &str) -> bool {
    use reqwest;
    use tokio::time::timeout;

    match timeout(
        WEBDRIVER_CHECK_TIMEOUT,
        reqwest::get(&format!("{}/status", url)),
    )
    .await
    {
        Ok(Ok(response)) => response.status().is_success(),
        _ => false,
    }
}

impl Default for BrowserManager {
    fn default() -> Self {
        Self::new()
    }
}
