use super::driver::{DriverConfig, DriverInfo, DriverManager, DriverType};
use crate::{
    Result,
    config::Config,
    constants::{
        BROWSER_LAUNCH_TIMEOUT, CHROME_DRIVER_ARGS, CHROMEDRIVER_DEFAULT_PORT, DEFAULT_TIMEOUT,
        FIREFOX_DRIVER_ARGS, GECKODRIVER_DEFAULT_PORT, WEBDRIVER_CHECK_TIMEOUT,
    },
    error::TarziError,
};
use std::{collections::HashMap, path::PathBuf};
use tempfile::TempDir;
use thirtyfour::{ChromiumLikeCapabilities, DesiredCapabilities, WebDriver};
use tracing::{error, info, warn};

/// Browser instance manager
#[derive(Debug)]
pub struct BrowserManager {
    browsers: HashMap<String, (WebDriver, TempDir)>,
    driver_manager: Option<DriverManager>,
    managed_driver_info: Option<DriverInfo>,
    config: Option<Config>,
}

impl BrowserManager {
    pub fn new() -> Self {
        Self {
            browsers: HashMap::new(),
            driver_manager: None,
            managed_driver_info: None,
            config: None,
        }
    }

    /// Create a new BrowserManager with configuration
    pub fn from_config(config: &Config) -> Self {
        Self {
            browsers: HashMap::new(),
            driver_manager: None,
            managed_driver_info: None,
            config: Some(config.clone()),
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
        // Determine which capabilities to use based on configuration

        let driver_type = if let Some(config) = &self.config {
            match config.fetcher.web_driver.as_str() {
                "geckodriver" | "firefox" => {
                    info!("Using Firefox capabilities for geckodriver");
                    "firefox"
                }
                "chromedriver" | "chrome" => {
                    info!("Using Chrome capabilities for chromedriver");
                    "chrome"
                }
                _ => {
                    info!("Unknown driver type, using Chrome capabilities as fallback");
                    "chrome"
                }
            }
        } else {
            info!("No configuration available, using Chrome capabilities as fallback");
            "chrome"
        };

        // Create capabilities based on driver type
        let browser_result = match driver_type {
            "firefox" => {
                let mut caps = DesiredCapabilities::firefox();
                if headless {
                    caps.add_arg("--headless").map_err(|e| {
                        error!("Failed to add headless arg: {}", e);
                        TarziError::Browser(format!("Failed to add headless arg: {e}"))
                    })?;
                }
                tokio::time::timeout(BROWSER_LAUNCH_TIMEOUT, WebDriver::new(&webdriver_url, caps))
                    .await
            }
            _ => {
                let mut caps = DesiredCapabilities::chrome();
                if headless {
                    caps.add_arg("--headless").map_err(|e| {
                        error!("Failed to add headless arg: {}", e);
                        TarziError::Browser(format!("Failed to add headless arg: {e}"))
                    })?;
                }
                caps.add_arg("--disable-gpu").map_err(|e| {
                    error!("Failed to add disable-gpu arg: {}", e);
                    TarziError::Browser(format!("Failed to add disable-gpu arg: {e}"))
                })?;
                caps.add_arg("--disable-dev-shm-usage").map_err(|e| {
                    error!("Failed to add disable-dev-shm-usage arg: {}", e);
                    TarziError::Browser(format!("Failed to add disable-dev-shm-usage arg: {e}"))
                })?;
                caps.add_arg("--no-sandbox").map_err(|e| {
                    error!("Failed to add no-sandbox arg: {}", e);
                    TarziError::Browser(format!("Failed to add no-sandbox arg: {e}"))
                })?;

                // Add proxy configuration if available
                if let Some(config) = &self.config {
                    let proxy = crate::config::get_proxy_from_env_or_config(&config.fetcher.proxy);
                    if let Some(proxy_url) = proxy {
                        if !proxy_url.is_empty() {
                            info!("Configuring browser with proxy: {}", proxy_url);
                            caps.add_arg(&format!("--proxy-server={proxy_url}"))
                                .map_err(|e| {
                                    error!("Failed to add proxy-server arg: {}", e);
                                    TarziError::Browser(format!(
                                        "Failed to add proxy-server arg: {e}"
                                    ))
                                })?;
                        }
                    }
                }

                tokio::time::timeout(BROWSER_LAUNCH_TIMEOUT, WebDriver::new(&webdriver_url, caps))
                    .await
            }
        };

        info!("Browser config created successfully");

        // Create temp directory for browser data
        let temp_dir = if let Some(_user_data_path) = user_data_dir {
            None
        } else {
            let temp = TempDir::new().map_err(|e| {
                error!("Failed to create temporary directory: {}", e);
                TarziError::Browser(format!("Failed to create temporary directory: {e}"))
            })?;
            Some(temp)
        };

        let browser = match browser_result {
            Ok(Ok(result)) => {
                info!("Browser launched successfully with ID: {}", instance_id);
                result
            }
            Ok(Err(e)) => {
                error!("Failed to create browser: {}", e);
                return Err(TarziError::Browser(format!(
                    "Failed to create browser: {e}"
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
                TarziError::Browser(format!("Failed to quit browser: {e}"))
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
            let instance_id = format!("{base_id}_{i}");
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

    /// Get or create a webdriver endpoint, using configuration or DriverManager
    /// Two mutually exclusive types:
    /// 1. External: configured by web_driver_url - if set, use it exclusively and fail if unavailable
    /// 2. Self-managed: managed by DriverManager - used only if web_driver_url is not set
    async fn get_or_create_webdriver_endpoint(&mut self) -> Result<String> {
        if let Some(config) = &self.config {
            if let Some(ref url) = config.fetcher.web_driver_url {
                if !url.is_empty() {
                    // External driver type: web_driver_url is explicitly configured
                    info!("Using external WebDriver URL from config: {}", url);
                    if is_webdriver_available_at_url(url).await {
                        info!(
                            "External WebDriver server is available and ready at: {}",
                            url
                        );
                        return Ok(url.clone());
                    } else {
                        error!(
                            "External WebDriver URL '{}' is configured but server is not available",
                            url
                        );
                        return Err(TarziError::Browser(format!(
                            "External WebDriver server is not available at configured URL: {url}. \
                         Please ensure the WebDriver server is running at this URL, or remove \
                         the web_driver_url configuration to use self-managed drivers."
                        )));
                    }
                }
            }
        }

        // Self-managed driver type: no web_driver_url configured, use DriverManager
        info!("No external WebDriver URL configured, using self-managed driver");

        // Try to find an already running WebDriver first (from previous self-managed instance)
        let default_url = if let Some(config) = &self.config {
            let web_driver = &config.fetcher.web_driver;
            match web_driver.as_str() {
                "geckodriver" | "firefox" => {
                    format!("http://localhost:{GECKODRIVER_DEFAULT_PORT}")
                }
                "chromedriver" | "chrome" => {
                    format!("http://localhost:{CHROMEDRIVER_DEFAULT_PORT}")
                }
                _ => {
                    // Fallback to GeckoDriver for unknown driver types
                    format!("http://localhost:{GECKODRIVER_DEFAULT_PORT}")
                }
            }
        } else {
            // No config available, use GeckoDriver as default
            format!("http://localhost:{GECKODRIVER_DEFAULT_PORT}")
        };

        info!(
            "Checking for existing self-managed WebDriver at: {}",
            default_url
        );
        if is_webdriver_available_at_url(&default_url).await {
            info!(
                "Found existing self-managed WebDriver server at: {}",
                default_url
            );
            return Ok(default_url);
        }

        // No existing WebDriver found, start one using DriverManager
        info!(
            "No existing WebDriver server found, starting self-managed driver using DriverManager"
        );

        // Initialize DriverManager if not already done
        if self.driver_manager.is_none() {
            info!("Initializing DriverManager for self-managed driver");
            self.driver_manager = Some(DriverManager::new());
        }

        // Try to start a driver using DriverManager
        let driver_manager = self.driver_manager.as_ref().unwrap();

        // Determine which driver to try first based on configuration
        let (primary_driver, fallback_driver) = if let Some(config) = &self.config {
            match config.fetcher.web_driver.as_str() {
                "geckodriver" | "firefox" => {
                    info!(
                        "Configuration specifies geckodriver, trying Firefox first for self-managed driver"
                    );
                    (DriverType::Firefox, DriverType::Chrome)
                }
                "chromedriver" | "chrome" => {
                    info!(
                        "Configuration specifies chromedriver, trying Chrome first for self-managed driver"
                    );
                    (DriverType::Chrome, DriverType::Firefox)
                }
                _ => {
                    info!(
                        "Unknown driver type in config, trying Firefox first for self-managed driver"
                    );
                    (DriverType::Firefox, DriverType::Chrome)
                }
            }
        } else {
            info!("No configuration available, trying Firefox first for self-managed driver");
            (DriverType::Firefox, DriverType::Chrome)
        };

        // Try the primary driver first
        match driver_manager.check_driver_binary(&primary_driver) {
            Ok(()) => {
                let (port, args) = match &primary_driver {
                    DriverType::Chrome => (CHROMEDRIVER_DEFAULT_PORT, CHROME_DRIVER_ARGS),
                    DriverType::Firefox => (GECKODRIVER_DEFAULT_PORT, FIREFOX_DRIVER_ARGS),
                    _ => (GECKODRIVER_DEFAULT_PORT, FIREFOX_DRIVER_ARGS),
                };

                let config = DriverConfig {
                    driver_type: primary_driver.clone(),
                    port,
                    args: args.iter().map(|s| s.to_string()).collect(),
                    timeout: DEFAULT_TIMEOUT,
                    verbose: false,
                };

                match driver_manager.start_driver_with_config(config) {
                    Ok(driver_info) => {
                        info!(
                            "Successfully started self-managed {:?} at: {}",
                            primary_driver, driver_info.endpoint
                        );
                        self.managed_driver_info = Some(driver_info.clone());
                        return Ok(driver_info.endpoint);
                    }
                    Err(e) => {
                        warn!(
                            "Failed to start self-managed {:?} with DriverManager: {}",
                            primary_driver, e
                        );
                    }
                }
            }
            Err(e) => {
                warn!("Self-managed {:?} not available: {}", primary_driver, e);
            }
        }

        // Try the fallback driver
        match driver_manager.check_driver_binary(&fallback_driver) {
            Ok(()) => {
                info!(
                    "Self-managed {:?} found, starting driver with DriverManager",
                    fallback_driver
                );
                let (port, args) = match &fallback_driver {
                    DriverType::Chrome => (CHROMEDRIVER_DEFAULT_PORT, CHROME_DRIVER_ARGS),
                    DriverType::Firefox => (GECKODRIVER_DEFAULT_PORT, FIREFOX_DRIVER_ARGS),
                    _ => (GECKODRIVER_DEFAULT_PORT, FIREFOX_DRIVER_ARGS),
                };

                let config = DriverConfig {
                    driver_type: fallback_driver.clone(),
                    port,
                    args: args.iter().map(|s| s.to_string()).collect(),
                    timeout: DEFAULT_TIMEOUT,
                    verbose: false,
                };

                match driver_manager.start_driver_with_config(config) {
                    Ok(driver_info) => {
                        info!(
                            "Successfully started self-managed {:?} at: {}",
                            fallback_driver, driver_info.endpoint
                        );
                        self.managed_driver_info = Some(driver_info.clone());
                        return Ok(driver_info.endpoint);
                    }
                    Err(e) => {
                        warn!(
                            "Failed to start self-managed {:?} with DriverManager: {}",
                            fallback_driver, e
                        );
                    }
                }
            }
            Err(e) => {
                warn!("Self-managed {:?} not available: {}", fallback_driver, e);
            }
        }

        // If all attempts failed, return an error with helpful guidance
        Err(TarziError::Browser(
            "No self-managed WebDriver could be started. Please either:\n\
            1. Install ChromeDriver (https://chromedriver.chromium.org/) or GeckoDriver (https://github.com/mozilla/geckodriver/releases) and ensure they're in your PATH, or\n\
            2. Configure web_driver_url in your tarzi.toml file to use an external WebDriver server".to_string()
        ))
    }

    /// Clean up managed driver if any
    pub async fn cleanup_managed_driver(&mut self) -> Result<()> {
        if let (Some(driver_manager), Some(driver_info)) =
            (&mut self.driver_manager, &self.managed_driver_info)
        {
            match driver_manager.stop_driver(driver_info.config.port) {
                Ok(()) => {
                    self.managed_driver_info = None;
                }
                Err(e) => {
                    warn!("Failed to stop managed driver: {}", e);
                    return Err(TarziError::Browser(format!(
                        "Failed to stop managed driver: {e}"
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

    /// Create a new browser instance with explicit proxy configuration
    pub async fn create_browser_with_proxy(
        &mut self,
        user_data_dir: Option<PathBuf>,
        headless: bool,
        instance_id: Option<String>,
        proxy: Option<String>,
    ) -> Result<String> {
        // Store original config proxy
        let original_proxy = self.config.as_ref().and_then(|c| c.fetcher.proxy.clone());

        // Temporarily override proxy configuration
        if let Some(config) = &mut self.config {
            config.fetcher.proxy = proxy;
        }

        // Create browser with proxy
        let result = self
            .create_browser_with_user_data(user_data_dir, headless, instance_id)
            .await;

        // Restore original proxy configuration
        if let Some(config) = &mut self.config {
            config.fetcher.proxy = original_proxy;
        }

        result
    }

    /// Asynchronously shut down all browser instances and managed driver
    pub async fn shutdown(&mut self) {
        // Clean up all browser instances
        let browser_ids: Vec<String> = self.browsers.keys().cloned().collect();
        for instance_id in browser_ids {
            if let Some((driver, _temp_dir)) = self.browsers.remove(&instance_id) {
                info!("Shutting down browser instance: {}", instance_id);
                if let Err(e) = driver.quit().await {
                    error!("Failed to quit browser instance {}: {}", instance_id, e);
                }
            }
        }
        // Clean up managed driver
        if let (Some(driver_manager), Some(driver_info)) =
            (&mut self.driver_manager, &self.managed_driver_info)
        {
            info!("Shutting down managed driver: {}", driver_info.endpoint);
            if let Err(e) = driver_manager.stop_driver(driver_info.config.port) {
                error!("Failed to stop managed driver: {}", e);
            }
            self.managed_driver_info = None;
        }
    }
}

impl Drop for BrowserManager {
    fn drop(&mut self) {
        if !self.browsers.is_empty() || self.managed_driver_info.is_some() {
            warn!(
                "BrowserManager dropped without explicit shutdown. Resources may not be cleaned up properly. Consider calling shutdown() before dropping."
            );
        }
    }
}

/// Helper function to check if webdriver is available at a specific URL
async fn is_webdriver_available_at_url(url: &str) -> bool {
    use reqwest;
    use tokio::time::timeout;

    match timeout(
        WEBDRIVER_CHECK_TIMEOUT,
        reqwest::get(&format!("{url}/status")),
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
