use crate::{Result, error::TarziError};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub fetcher: FetcherConfig,
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
pub struct FetcherConfig {
    #[serde(default = "default_fetcher_mode")]
    pub mode: String,
    #[serde(default = "default_fetcher_format")]
    pub format: String,
    #[serde(default = "default_user_agent")]
    pub user_agent: String,
    #[serde(default = "default_fetch_timeout")]
    pub timeout: u64,
    pub proxy: Option<String>,
    #[serde(default = "default_web_driver")]
    pub web_driver: String,
    pub web_driver_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    #[serde(default = "default_search_mode")]
    pub mode: String,
    #[serde(default = "default_search_engine")]
    pub engine: String,
    #[serde(default = "default_query_pattern")]
    pub query_pattern: String,
    #[serde(default = "default_result_limit")]
    pub limit: usize,
    #[serde(default = "default_autoswitch_strategy")]
    pub autoswitch: String,
    pub api_key: Option<String>,
    pub brave_api_key: Option<String>,
    pub duckduckgo_api_key: Option<String>,
    pub google_serper_api_key: Option<String>,
    pub exa_api_key: Option<String>,
    pub travily_api_key: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            general: GeneralConfig::default(),
            fetcher: FetcherConfig::default(),
            search: SearchConfig::default(),
        }
    }

    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .map_err(|e| TarziError::Config(format!("Failed to read config file: {e}")))?;

            let config: Config = toml::from_str(&content)
                .map_err(|e| TarziError::Config(format!("Failed to parse config file: {e}")))?;

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
            fs::create_dir_all(parent).map_err(|e| {
                TarziError::Config(format!("Failed to create config directory: {e}"))
            })?;
        }

        let content = toml::to_string_pretty(self)
            .map_err(|e| TarziError::Config(format!("Failed to serialize config: {e}")))?;

        fs::write(&config_path, content)
            .map_err(|e| TarziError::Config(format!("Failed to write config file: {e}")))?;

        Ok(())
    }

    fn get_config_path() -> Result<PathBuf> {
        let home_dir = std::env::var("HOME")
            .map_err(|_| TarziError::Config("HOME environment variable not set".to_string()))?;

        Ok(PathBuf::from(home_dir).join(".tarzi.toml"))
    }

    pub fn get_dev_config_path() -> PathBuf {
        PathBuf::from("tarzi.toml")
    }

    pub fn load_dev() -> Result<Self> {
        let config_path = Self::get_dev_config_path();

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .map_err(|e| TarziError::Config(format!("Failed to read dev config file: {e}")))?;

            let config: Config = toml::from_str(&content)
                .map_err(|e| TarziError::Config(format!("Failed to parse dev config file: {e}")))?;

            Ok(config)
        } else {
            // Return default config if file doesn't exist
            Ok(Config::new())
        }
    }

    pub fn save_dev(&self) -> Result<()> {
        let config_path = Self::get_dev_config_path();

        let content = toml::to_string_pretty(self)
            .map_err(|e| TarziError::Config(format!("Failed to serialize dev config: {e}")))?;

        fs::write(&config_path, content)
            .map_err(|e| TarziError::Config(format!("Failed to write dev config file: {e}")))?;

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

impl Default for FetcherConfig {
    fn default() -> Self {
        Self {
            mode: default_fetcher_mode(),
            format: default_fetcher_format(),
            user_agent: default_user_agent(),
            timeout: default_fetch_timeout(),
            proxy: None,
            web_driver: default_web_driver(),
            web_driver_url: None,
        }
    }
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            mode: default_search_mode(),
            engine: default_search_engine(),
            query_pattern: default_query_pattern(),
            limit: default_result_limit(),
            autoswitch: default_autoswitch_strategy(),
            api_key: None,
            brave_api_key: None,
            duckduckgo_api_key: None,
            google_serper_api_key: None,
            exa_api_key: None,
            travily_api_key: None,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

// Default value functions
fn default_log_level() -> String {
    "info".to_string()
}

fn default_timeout() -> u64 {
    30
}

fn default_fetcher_mode() -> String {
    "browser_headless".to_string()
}

fn default_fetcher_format() -> String {
    "markdown".to_string()
}

fn default_user_agent() -> String {
    crate::constants::DEFAULT_USER_AGENT.to_string()
}

fn default_fetch_timeout() -> u64 {
    30
}

fn default_search_mode() -> String {
    "webquery".to_string()
}

fn default_search_engine() -> String {
    "bing".to_string()
}

fn default_query_pattern() -> String {
    "https://www.bing.com/search?q={query}".to_string()
}

fn default_result_limit() -> usize {
    5
}

fn default_web_driver() -> String {
    "geckodriver".to_string()
}

fn default_autoswitch_strategy() -> String {
    "smart".to_string()
}

/// Get proxy configuration with environment variable override
/// Environment variables checked in order: HTTP_PROXY, HTTPS_PROXY, http_proxy, https_proxy
/// Falls back to config.proxy if no environment variables are set
pub fn get_proxy_from_env_or_config(config_proxy: &Option<String>) -> Option<String> {
    // Check environment variables in order of preference
    let env_vars = ["HTTPS_PROXY", "HTTP_PROXY", "https_proxy", "http_proxy"];

    for env_var in &env_vars {
        if let Ok(proxy) = std::env::var(env_var) {
            if !proxy.is_empty() {
                return Some(proxy);
            }
        }
    }

    // Fall back to config proxy
    config_proxy.clone()
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
        assert_eq!(config.fetcher.mode, "browser_headless");
        assert_eq!(config.fetcher.format, "markdown");
        assert_eq!(
            config.fetcher.user_agent,
            crate::constants::DEFAULT_USER_AGENT
        );
        assert_eq!(config.fetcher.timeout, 30);
        assert_eq!(config.search.mode, "webquery");
        assert_eq!(config.search.engine, "bing");
        assert_eq!(
            config.search.query_pattern,
            "https://www.bing.com/search?q={query}"
        );
        assert_eq!(config.search.limit, 5);
        assert!(config.search.api_key.is_none());
    }

    #[test]
    fn test_config_serialization() {
        let mut config = Config::new();
        config.search.api_key = Some("test_key".to_string());
        config.search.limit = 5;
        config.fetcher.mode = "head".to_string();

        let toml_str = toml::to_string_pretty(&config).unwrap();
        let parsed_config: Config = toml::from_str(&toml_str).unwrap();

        assert_eq!(parsed_config.search.api_key, Some("test_key".to_string()));
        assert_eq!(parsed_config.search.limit, 5);
        assert_eq!(parsed_config.fetcher.mode, "head");
    }

    #[test]
    fn test_config_save_and_load() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");

        // Create a test config
        let mut config = Config::new();
        config.search.api_key = Some("test_api_key".to_string());
        config.search.brave_api_key = Some("test_brave_key".to_string());
        config.search.limit = 10;
        config.general.log_level = "debug".to_string();

        // Save config to temporary file
        let content = toml::to_string_pretty(&config).unwrap();
        fs::write(&config_path, content).unwrap();

        // Load config from file
        let content = fs::read_to_string(&config_path).unwrap();
        let loaded_config: Config = toml::from_str(&content).unwrap();

        assert_eq!(
            loaded_config.search.api_key,
            Some("test_api_key".to_string())
        );
        assert_eq!(
            loaded_config.search.brave_api_key,
            Some("test_brave_key".to_string())
        );
        assert_eq!(loaded_config.search.limit, 10);
        assert_eq!(loaded_config.general.log_level, "debug");
    }

    #[test]
    fn test_dev_config_path() {
        let dev_path = Config::get_dev_config_path();
        assert_eq!(dev_path, PathBuf::from("tarzi.toml"));
    }

    #[test]
    fn test_config_with_custom_values() {
        let config_str = r#"
[general]
log_level = "debug"
timeout = 60

[fetcher]
mode = "head"
format = "json"
user_agent = "Custom User Agent"
timeout = 45
proxy = "http://example.com:8080"
web_driver = "chrome"
web_driver_url = "http://example.com/driver"

[search]
mode = "api"
engine = "google.com"
query_pattern = ".*"
limit = 5
autoswitch = "none"
api_key = "google_key_123"
brave_api_key = "brave_key_456"
google_serper_api_key = "serper_key_789"
exa_api_key = "exa_key_012"
travily_api_key = "travily_key_345"
"#;

        let config: Config = toml::from_str(config_str).unwrap();

        assert_eq!(config.general.log_level, "debug");
        assert_eq!(config.general.timeout, 60);
        assert_eq!(config.fetcher.mode, "head");
        assert_eq!(config.fetcher.format, "json");
        assert_eq!(config.fetcher.user_agent, "Custom User Agent");
        assert_eq!(config.fetcher.timeout, 45);
        assert_eq!(
            config.fetcher.proxy,
            Some("http://example.com:8080".to_string())
        );
        assert_eq!(config.fetcher.web_driver, "chrome");
        assert_eq!(
            config.fetcher.web_driver_url,
            Some("http://example.com/driver".to_string())
        );
        assert_eq!(config.search.mode, "api");
        assert_eq!(config.search.engine, "google.com");
        assert_eq!(config.search.query_pattern, ".*");
        assert_eq!(config.search.limit, 5);
        assert_eq!(config.search.autoswitch, "none");
        assert_eq!(config.search.api_key, Some("google_key_123".to_string()));
        assert_eq!(config.search.brave_api_key, Some("brave_key_456".to_string()));
        assert_eq!(config.search.google_serper_api_key, Some("serper_key_789".to_string()));
        assert_eq!(config.search.exa_api_key, Some("exa_key_012".to_string()));
        assert_eq!(config.search.travily_api_key, Some("travily_key_345".to_string()));
    }

    #[test]
    fn test_config_with_only_web_driver_url() {
        let config_str = r#"
[fetcher]
web_driver_url = "http://localhost:9999"
"#;
        let config: Config = toml::from_str(config_str).unwrap();
        // Should use default for web_driver
        assert_eq!(config.fetcher.web_driver, "geckodriver");
        assert_eq!(
            config.fetcher.web_driver_url,
            Some("http://localhost:9999".to_string())
        );
    }

    #[test]
    fn test_load_actual_tarzi_toml() {
        // Test loading the actual tarzi.toml file
        let config = Config::load_dev();
        assert!(
            config.is_ok(),
            "Failed to load tarzi.toml: {:?}",
            config.err()
        );

        let config = config.unwrap();

        // Verify the structure matches our expectations
        assert_eq!(config.general.log_level, "info");
        assert_eq!(config.general.timeout, 30);
        assert_eq!(config.fetcher.mode, "browser_headless");
        assert_eq!(config.fetcher.format, "markdown");
        assert_eq!(
            config.fetcher.user_agent,
            crate::constants::DEFAULT_USER_AGENT
        );
        assert_eq!(config.fetcher.timeout, 30);
        // Proxy should be None by default (commented out in tarzi.toml)
        assert_eq!(config.fetcher.proxy, None);
        assert_eq!(config.search.mode, "webquery");
        assert_eq!(config.search.engine, "bing");
        assert_eq!(
            config.search.query_pattern,
            "https://www.bing.com/search?q={query}"
        );
        assert_eq!(config.search.limit, 5);
        assert_eq!(config.search.autoswitch, "smart");
        // api_key should be None by default (commented out in tarzi.toml)
        assert_eq!(config.search.api_key, None);
        assert_eq!(config.search.brave_api_key, None);
        assert_eq!(config.search.duckduckgo_api_key, None);
        assert_eq!(config.search.google_serper_api_key, None);
        assert_eq!(config.search.exa_api_key, None);
        assert_eq!(config.search.travily_api_key, None);
    }

    #[test]
    fn test_get_proxy_from_env_or_config() {
        use std::sync::Mutex;

        // Use a static mutex to serialize access to environment variables across tests
        static ENV_LOCK: Mutex<()> = Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap();

        // Store original environment variables
        let original_http_proxy = std::env::var("HTTP_PROXY").ok();
        let original_https_proxy = std::env::var("HTTPS_PROXY").ok();
        let original_http_proxy_lower = std::env::var("http_proxy").ok();
        let original_https_proxy_lower = std::env::var("https_proxy").ok();

        // Clean up any existing environment variables first
        unsafe {
            std::env::remove_var("HTTP_PROXY");
            std::env::remove_var("HTTPS_PROXY");
            std::env::remove_var("http_proxy");
            std::env::remove_var("https_proxy");
        }

        // Test with no environment variables and no config proxy
        let result = get_proxy_from_env_or_config(&None);
        assert_eq!(result, None);

        // Test with config proxy but no environment variables
        let config_proxy = Some("http://config-proxy:8080".to_string());
        let result = get_proxy_from_env_or_config(&config_proxy);
        assert_eq!(result, config_proxy);

        // Test with environment variable (HTTP_PROXY)
        unsafe {
            std::env::set_var("HTTP_PROXY", "http://env-proxy:8080");
        }
        let result = get_proxy_from_env_or_config(&config_proxy);
        assert_eq!(result, Some("http://env-proxy:8080".to_string()));

        // Test with HTTPS_PROXY (should take precedence over HTTP_PROXY)
        unsafe {
            std::env::set_var("HTTPS_PROXY", "http://https-proxy:8080");
        }
        let result = get_proxy_from_env_or_config(&config_proxy);
        assert_eq!(result, Some("http://https-proxy:8080".to_string()));

        // Test with lowercase environment variable (remove uppercase first)
        unsafe {
            std::env::remove_var("HTTP_PROXY");
            std::env::remove_var("HTTPS_PROXY");
            std::env::set_var("http_proxy", "http://lowercase-proxy:8080");
        }
        let result = get_proxy_from_env_or_config(&config_proxy);
        assert_eq!(result, Some("http://lowercase-proxy:8080".to_string()));

        // Clean up and restore original environment variables
        unsafe {
            std::env::remove_var("HTTP_PROXY");
            std::env::remove_var("HTTPS_PROXY");
            std::env::remove_var("http_proxy");
            std::env::remove_var("https_proxy");

            // Restore original values
            if let Some(val) = original_http_proxy {
                std::env::set_var("HTTP_PROXY", val);
            }
            if let Some(val) = original_https_proxy {
                std::env::set_var("HTTPS_PROXY", val);
            }
            if let Some(val) = original_http_proxy_lower {
                std::env::set_var("http_proxy", val);
            }
            if let Some(val) = original_https_proxy_lower {
                std::env::set_var("https_proxy", val);
            }
        }
    }

    #[test]
    fn test_get_proxy_from_env_or_config_empty_env() {
        use std::sync::Mutex;

        // Use a static mutex to serialize access to environment variables across tests
        static ENV_LOCK: Mutex<()> = Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap();

        // Store original environment variables
        let original_http_proxy = std::env::var("HTTP_PROXY").ok();
        let original_https_proxy = std::env::var("HTTPS_PROXY").ok();
        let original_http_proxy_lower = std::env::var("http_proxy").ok();
        let original_https_proxy_lower = std::env::var("https_proxy").ok();

        // Clean up any existing environment variables first
        unsafe {
            std::env::remove_var("HTTP_PROXY");
            std::env::remove_var("HTTPS_PROXY");
            std::env::remove_var("http_proxy");
            std::env::remove_var("https_proxy");
        }

        // Test with empty environment variable (should fall back to config)
        unsafe {
            std::env::set_var("HTTP_PROXY", "");
        }
        let config_proxy = Some("http://config-proxy:8080".to_string());
        let result = get_proxy_from_env_or_config(&config_proxy);
        assert_eq!(result, config_proxy);

        // Clean up and restore original environment variables
        unsafe {
            std::env::remove_var("HTTP_PROXY");
            std::env::remove_var("HTTPS_PROXY");
            std::env::remove_var("http_proxy");
            std::env::remove_var("https_proxy");

            // Restore original values
            if let Some(val) = original_http_proxy {
                std::env::set_var("HTTP_PROXY", val);
            }
            if let Some(val) = original_https_proxy {
                std::env::set_var("HTTPS_PROXY", val);
            }
            if let Some(val) = original_http_proxy_lower {
                std::env::set_var("http_proxy", val);
            }
            if let Some(val) = original_https_proxy_lower {
                std::env::set_var("https_proxy", val);
            }
        }
    }
}
