use crate::{Result, error::TarziError, utils::is_webdriver_available};
use std::{collections::HashMap, path::PathBuf, time::Duration};
use tempfile::TempDir;
use thirtyfour::{ChromiumLikeCapabilities, DesiredCapabilities, WebDriver};
use tracing::{error, info, warn};

/// Browser instance manager
pub struct BrowserManager {
    browsers: HashMap<String, (WebDriver, TempDir)>,
}

impl BrowserManager {
    pub fn new() -> Self {
        Self {
            browsers: HashMap::new(),
        }
    }

    /// Create a new browser instance with a specific user data directory
    pub async fn create_browser_with_user_data(
        &mut self,
        user_data_dir: Option<PathBuf>,
        headless: bool,
        instance_id: Option<String>,
    ) -> Result<String> {
        let webdriver_url = std::env::var("TARZI_WEBDRIVER_URL")
            .unwrap_or_else(|_| "http://localhost:4444".to_string());

        // Check if WebDriver server is available before creating browser
        info!("Checking WebDriver availability at: {}", webdriver_url);
        if !is_webdriver_available().await {
            return Err(TarziError::Browser(format!(
                "WebDriver server is not available at {}. Please start ChromeDriver or another WebDriver server.",
                webdriver_url
            )));
        }
        info!("WebDriver server is available and ready");

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
        let browser_result = tokio::time::timeout(
            Duration::from_secs(60),
            WebDriver::new(&webdriver_url, caps),
        )
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
}

impl Default for BrowserManager {
    fn default() -> Self {
        Self::new()
    }
}
