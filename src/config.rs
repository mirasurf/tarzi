use crate::{error::TarsierError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub converter: ConverterConfig,
    #[serde(default)]
    pub fetcher: FetcherConfig,
    #[serde(default)]
    pub browser: BrowserConfig,
    #[serde(default)]
    pub search: SearchConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConverterConfig {
    #[serde(default = "default_output_format")]
    pub default_format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetcherConfig {
    #[serde(default = "default_user_agent")]
    pub user_agent: String,
    #[serde(default = "default_fetch_timeout")]
    pub timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserConfig {
    #[serde(default = "default_browser_mode")]
    pub browser_mode: String,
    #[serde(default = "default_browser_timeout")]
    pub timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    #[serde(default = "default_search_mode")]
    pub search_mode: String,
    #[serde(default = "default_search_engine")]
    pub search_engine: String,
    #[serde(default = "default_result_limit")]
    pub result_limit: usize,
    pub google_search_api_key: Option<String>,
    pub bing_search_api_key: Option<String>,
    pub duckduckgo_api_key: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            general: GeneralConfig::default(),
            converter: ConverterConfig::default(),
            fetcher: FetcherConfig::default(),
            browser: BrowserConfig::default(),
            search: SearchConfig::default(),
        }
    }

    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .map_err(|e| TarsierError::Config(format!("Failed to read config file: {}", e)))?;
            
            let config: Config = toml::from_str(&content)
                .map_err(|e| TarsierError::Config(format!("Failed to parse config file: {}", e)))?;
            
            Ok(config)
        } else {
            // Return default config if file doesn't exist
            Ok(Config::new())
        }
    }

    pub fn load_or_create() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        
        if config_path.exists() {
            Self::load()
        } else {
            // Create default config file
            let config = Config::new();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| TarsierError::Config(format!("Failed to create config directory: {}", e)))?;
        }
        
        let content = toml::to_string_pretty(self)
            .map_err(|e| TarsierError::Config(format!("Failed to serialize config: {}", e)))?;
        
        fs::write(&config_path, content)
            .map_err(|e| TarsierError::Config(format!("Failed to write config file: {}", e)))?;
        
        Ok(())
    }

    fn get_config_path() -> Result<PathBuf> {
        let home_dir = std::env::var("HOME")
            .map_err(|_| TarsierError::Config("HOME environment variable not set".to_string()))?;
        
        Ok(PathBuf::from(home_dir).join(".tarsier.toml"))
    }

    pub fn get_dev_config_path() -> PathBuf {
        PathBuf::from("tarsier.toml")
    }

    pub fn load_dev() -> Result<Self> {
        let config_path = Self::get_dev_config_path();
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .map_err(|e| TarsierError::Config(format!("Failed to read dev config file: {}", e)))?;
            
            let config: Config = toml::from_str(&content)
                .map_err(|e| TarsierError::Config(format!("Failed to parse dev config file: {}", e)))?;
            
            Ok(config)
        } else {
            // Return default config if file doesn't exist
            Ok(Config::new())
        }
    }

    pub fn save_dev(&self) -> Result<()> {
        let config_path = Self::get_dev_config_path();
        
        let content = toml::to_string_pretty(self)
            .map_err(|e| TarsierError::Config(format!("Failed to serialize dev config: {}", e)))?;
        
        fs::write(&config_path, content)
            .map_err(|e| TarsierError::Config(format!("Failed to write dev config file: {}", e)))?;
        
        Ok(())
    }
}

// Default implementations
impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            log_level: default_log_level(),
            timeout: default_timeout(),
        }
    }
}

impl Default for ConverterConfig {
    fn default() -> Self {
        Self {
            default_format: default_output_format(),
        }
    }
}

impl Default for FetcherConfig {
    fn default() -> Self {
        Self {
            user_agent: default_user_agent(),
            timeout: default_fetch_timeout(),
        }
    }
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            browser_mode: default_browser_mode(),
            timeout: default_browser_timeout(),
        }
    }
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            search_mode: default_search_mode(),
            search_engine: default_search_engine(),
            result_limit: default_result_limit(),
            google_search_api_key: None,
            bing_search_api_key: None,
            duckduckgo_api_key: None,
        }
    }
}

// Default value functions
fn default_log_level() -> String {
    "info".to_string()
}

fn default_timeout() -> u64 {
    30
}

fn default_output_format() -> String {
    "markdown".to_string()
}

fn default_user_agent() -> String {
    "Mozilla/5.0 (compatible; Tarsier/1.0)".to_string()
}

fn default_fetch_timeout() -> u64 {
    30
}

fn default_browser_mode() -> String {
    "headless".to_string()
}

fn default_browser_timeout() -> u64 {
    60
}

fn default_search_mode() -> String {
    "browser".to_string()
}

fn default_search_engine() -> String {
    "bing.com".to_string()
}

fn default_result_limit() -> usize {
    3
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = Config::new();
        
        assert_eq!(config.general.log_level, "info");
        assert_eq!(config.general.timeout, 30);
        assert_eq!(config.converter.default_format, "markdown");
        assert_eq!(config.fetcher.user_agent, "Mozilla/5.0 (compatible; Tarsier/1.0)");
        assert_eq!(config.fetcher.timeout, 30);
        assert_eq!(config.browser.browser_mode, "headless");
        assert_eq!(config.browser.timeout, 60);
        assert_eq!(config.search.search_mode, "browser");
        assert_eq!(config.search.search_engine, "bing.com");
        assert_eq!(config.search.result_limit, 3);
        assert!(config.search.google_search_api_key.is_none());
        assert!(config.search.bing_search_api_key.is_none());
        assert!(config.search.duckduckgo_api_key.is_none());
    }

    #[test]
    fn test_config_serialization() {
        let mut config = Config::new();
        config.search.google_search_api_key = Some("test_key".to_string());
        config.search.result_limit = 5;
        config.browser.browser_mode = "head".to_string();
        
        let toml_str = toml::to_string_pretty(&config).unwrap();
        let parsed_config: Config = toml::from_str(&toml_str).unwrap();
        
        assert_eq!(parsed_config.search.google_search_api_key, Some("test_key".to_string()));
        assert_eq!(parsed_config.search.result_limit, 5);
        assert_eq!(parsed_config.browser.browser_mode, "head");
    }

    #[test]
    fn test_config_save_and_load() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");
        
        // Create a test config
        let mut config = Config::new();
        config.search.google_search_api_key = Some("test_api_key".to_string());
        config.search.result_limit = 10;
        config.general.log_level = "debug".to_string();
        
        // Save config to temporary file
        let content = toml::to_string_pretty(&config).unwrap();
        fs::write(&config_path, content).unwrap();
        
        // Load config from file
        let content = fs::read_to_string(&config_path).unwrap();
        let loaded_config: Config = toml::from_str(&content).unwrap();
        
        assert_eq!(loaded_config.search.google_search_api_key, Some("test_api_key".to_string()));
        assert_eq!(loaded_config.search.result_limit, 10);
        assert_eq!(loaded_config.general.log_level, "debug");
    }

    #[test]
    fn test_dev_config_path() {
        let dev_path = Config::get_dev_config_path();
        assert_eq!(dev_path, PathBuf::from("tarsier.toml"));
    }

    #[test]
    fn test_config_with_custom_values() {
        let config_str = r#"
[general]
log_level = "debug"
timeout = 60

[converter]
default_format = "json"

[fetcher]
user_agent = "Custom User Agent"
timeout = 45

[browser]
browser_mode = "head"
timeout = 90

[search]
search_mode = "api"
search_engine = "google.com"
result_limit = 5
google_search_api_key = "google_key_123"
bing_search_api_key = "bing_key_456"
duckduckgo_api_key = "ddg_key_789"
"#;

        let config: Config = toml::from_str(config_str).unwrap();
        
        assert_eq!(config.general.log_level, "debug");
        assert_eq!(config.general.timeout, 60);
        assert_eq!(config.converter.default_format, "json");
        assert_eq!(config.fetcher.user_agent, "Custom User Agent");
        assert_eq!(config.fetcher.timeout, 45);
        assert_eq!(config.browser.browser_mode, "head");
        assert_eq!(config.browser.timeout, 90);
        assert_eq!(config.search.search_mode, "api");
        assert_eq!(config.search.search_engine, "google.com");
        assert_eq!(config.search.result_limit, 5);
        assert_eq!(config.search.google_search_api_key, Some("google_key_123".to_string()));
        assert_eq!(config.search.bing_search_api_key, Some("bing_key_456".to_string()));
        assert_eq!(config.search.duckduckgo_api_key, Some("ddg_key_789".to_string()));
    }
} 