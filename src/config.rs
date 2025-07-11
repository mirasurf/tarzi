use crate::constants::{
    AUTOSWITCH_STRATEGY_SMART, DEFAULT_QUERY_PATTERN, DEFAULT_SEARCH_LIMIT, DEFAULT_TIMEOUT_SECS,
    FETCHER_MODE_BROWSER_HEADLESS, FORMAT_MARKDOWN, LOG_LEVEL_INFO, SEARCH_ENGINE_DUCKDUCKGO,
    SEARCH_MODE_WEBQUERY,
};
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
    pub brave_api_key: Option<String>,
    pub exa_api_key: Option<String>,
    pub travily_api_key: Option<String>,
    pub baidu_api_key: Option<String>,
}

/// CLI configuration parameters that can override config file values
#[derive(Debug, Clone)]
pub struct CliConfigParams {
    pub fetcher_format: Option<String>,
    pub search_limit: Option<usize>,
    pub search_engine: Option<String>,
}

impl CliConfigParams {
    pub fn new() -> Self {
        Self {
            fetcher_format: None,
            search_limit: None,
            search_engine: None,
        }
    }
}

impl Default for CliConfigParams {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            general: GeneralConfig::default(),
            fetcher: FetcherConfig::default(),
            search: SearchConfig::default(),
        }
    }

    /// Load configuration with proper precedence order:
    /// 1. CLI parameters (highest priority)
    /// 2. ~/.tarzi.toml (user config)
    /// 3. tarzi.toml (project config)
    /// 4. Default values (lowest priority)
    pub fn load_with_precedence() -> Result<Self> {
        // Start with default config
        let mut config = Config::new();

        // Load from project config (tarzi.toml) if it exists
        let project_config = Self::load_dev();
        if let Ok(project_config) = project_config {
            config.merge(&project_config);
        }

        // Load from user config (~/.tarzi.toml) if it exists (overrides project config)
        let user_config = Self::load();
        if let Ok(user_config) = user_config {
            config.merge(&user_config);
        }

        Ok(config)
    }

    /// Merge another config into this one (other config takes precedence)
    pub fn merge(&mut self, other: &Config) {
        // Merge general config
        if other.general.log_level != default_log_level() {
            self.general.log_level = other.general.log_level.clone();
        }
        if other.general.timeout != default_timeout() {
            self.general.timeout = other.general.timeout;
        }

        // Merge fetcher config
        if other.fetcher.mode != default_fetcher_mode() {
            self.fetcher.mode = other.fetcher.mode.clone();
        }
        if other.fetcher.format != default_fetcher_format() {
            self.fetcher.format = other.fetcher.format.clone();
        }
        if other.fetcher.user_agent != default_user_agent() {
            self.fetcher.user_agent = other.fetcher.user_agent.clone();
        }
        if other.fetcher.timeout != default_fetch_timeout() {
            self.fetcher.timeout = other.fetcher.timeout;
        }
        if other.fetcher.proxy.is_some() {
            self.fetcher.proxy = other.fetcher.proxy.clone();
        }
        if other.fetcher.web_driver != default_web_driver() {
            self.fetcher.web_driver = other.fetcher.web_driver.clone();
        }
        if other.fetcher.web_driver_url.is_some() {
            self.fetcher.web_driver_url = other.fetcher.web_driver_url.clone();
        }

        // Merge search config
        if other.search.mode != default_search_mode() {
            self.search.mode = other.search.mode.clone();
        }
        if other.search.engine != default_search_engine() {
            self.search.engine = other.search.engine.clone();
        }
        if other.search.query_pattern != default_query_pattern() {
            self.search.query_pattern = other.search.query_pattern.clone();
        }
        if other.search.limit != default_result_limit() {
            self.search.limit = other.search.limit;
        }
        if other.search.autoswitch != default_autoswitch_strategy() {
            self.search.autoswitch = other.search.autoswitch.clone();
        }
        if other.search.brave_api_key.is_some() {
            self.search.brave_api_key = other.search.brave_api_key.clone();
        }
        if other.search.exa_api_key.is_some() {
            self.search.exa_api_key = other.search.exa_api_key.clone();
        }
        if other.search.travily_api_key.is_some() {
            self.search.travily_api_key = other.search.travily_api_key.clone();
        }
        if other.search.baidu_api_key.is_some() {
            self.search.baidu_api_key = other.search.baidu_api_key.clone();
        }
    }

    /// Apply CLI parameters to config (highest priority)
    pub fn apply_cli_params(&mut self, cli_params: &CliConfigParams) {
        if let Some(format) = &cli_params.fetcher_format {
            self.fetcher.format = format.clone();
        }
        if let Some(limit) = cli_params.search_limit {
            self.search.limit = limit;
        }
        if let Some(engine) = &cli_params.search_engine {
            self.search.engine = engine.clone();
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
            brave_api_key: None,
            exa_api_key: None,
            travily_api_key: None,
            baidu_api_key: None,
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
    LOG_LEVEL_INFO.to_string()
}

fn default_timeout() -> u64 {
    DEFAULT_TIMEOUT_SECS
}

fn default_fetcher_mode() -> String {
    FETCHER_MODE_BROWSER_HEADLESS.to_string()
}

fn default_fetcher_format() -> String {
    FORMAT_MARKDOWN.to_string()
}

fn default_user_agent() -> String {
    crate::constants::DEFAULT_USER_AGENT.to_string()
}

fn default_fetch_timeout() -> u64 {
    30
}

fn default_search_mode() -> String {
    SEARCH_MODE_WEBQUERY.to_string()
}

fn default_search_engine() -> String {
    SEARCH_ENGINE_DUCKDUCKGO.to_string()
}

fn default_query_pattern() -> String {
    DEFAULT_QUERY_PATTERN.to_string()
}

fn default_result_limit() -> usize {
    DEFAULT_SEARCH_LIMIT
}

fn default_web_driver() -> String {
    "geckodriver".to_string()
}

fn default_autoswitch_strategy() -> String {
    AUTOSWITCH_STRATEGY_SMART.to_string()
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
    use crate::constants::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = Config::new();

        assert_eq!(config.general.log_level, LOG_LEVEL_INFO);
        assert_eq!(config.general.timeout, DEFAULT_TIMEOUT_SECS);
        assert_eq!(config.fetcher.mode, FETCHER_MODE_BROWSER_HEADLESS);
        assert_eq!(config.fetcher.format, FORMAT_MARKDOWN);
        assert_eq!(
            config.fetcher.user_agent,
            crate::constants::DEFAULT_USER_AGENT
        );
        assert_eq!(config.fetcher.timeout, 30);
        assert_eq!(config.search.mode, SEARCH_MODE_WEBQUERY);
        assert_eq!(config.search.engine, SEARCH_ENGINE_DUCKDUCKGO);
        assert_eq!(config.search.query_pattern, DEFAULT_QUERY_PATTERN);
        assert_eq!(config.search.limit, DEFAULT_SEARCH_LIMIT);
    }

    #[test]
    fn test_config_serialization() {
        let mut config = Config::new();
        config.search.brave_api_key = Some("test_key".to_string());
        config.search.limit = DEFAULT_SEARCH_LIMIT;
        config.fetcher.mode = FETCHER_MODE_HEAD.to_string();

        let toml_str = toml::to_string_pretty(&config).unwrap();
        let parsed_config: Config = toml::from_str(&toml_str).unwrap();

        assert_eq!(
            parsed_config.search.brave_api_key,
            Some("test_key".to_string())
        );
        assert_eq!(parsed_config.search.limit, DEFAULT_SEARCH_LIMIT);
        assert_eq!(parsed_config.fetcher.mode, FETCHER_MODE_HEAD);
    }

    #[test]
    fn test_config_save_and_load() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");

        // Create a test config
        let mut config = Config::new();
        config.search.brave_api_key = Some("test_brave_key".to_string());
        config.search.limit = DEFAULT_SEARCH_LIMIT;
        config.general.log_level = LOG_LEVEL_DEBUG.to_string();

        // Save config to temporary file
        let content = toml::to_string_pretty(&config).unwrap();
        fs::write(&config_path, content).unwrap();

        // Load config from file
        let content = fs::read_to_string(&config_path).unwrap();
        let loaded_config: Config = toml::from_str(&content).unwrap();

        assert_eq!(
            loaded_config.search.brave_api_key,
            Some("test_brave_key".to_string())
        );
        assert_eq!(loaded_config.search.limit, DEFAULT_SEARCH_LIMIT);
        assert_eq!(loaded_config.general.log_level, LOG_LEVEL_DEBUG);
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
brave_api_key = "brave_key_456"
exa_api_key = "exa_key_012"
travily_api_key = "travily_key_345"
"#;

        let config: Config = toml::from_str(config_str).unwrap();

        assert_eq!(config.general.log_level, "debug");
        assert_eq!(config.general.timeout, 60);
        assert_eq!(config.fetcher.mode, FETCHER_MODE_HEAD);
        assert_eq!(config.fetcher.format, FORMAT_JSON);
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
        assert_eq!(config.search.autoswitch, AUTOSWITCH_STRATEGY_NONE);

        assert_eq!(
            config.search.brave_api_key,
            Some("brave_key_456".to_string())
        );
        assert_eq!(config.search.exa_api_key, Some("exa_key_012".to_string()));
        assert_eq!(
            config.search.travily_api_key,
            Some("travily_key_345".to_string())
        );
    }

    #[test]
    fn test_config_with_only_web_driver_url() {
        let config_str = r#"
[fetcher]
web_driver_url = "http://localhost:9999"
"#;
        let config: Config = toml::from_str(config_str).unwrap();
        // Should use default for web_driver
        assert_eq!(config.fetcher.web_driver, GECKODRIVER);
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
        assert_eq!(config.general.log_level, LOG_LEVEL_INFO);
        assert_eq!(config.general.timeout, DEFAULT_TIMEOUT_SECS);
        assert_eq!(config.fetcher.mode, FETCHER_MODE_BROWSER_HEADLESS);
        assert_eq!(config.fetcher.format, FORMAT_MARKDOWN);
        assert_eq!(
            config.fetcher.user_agent,
            crate::constants::DEFAULT_USER_AGENT
        );
        assert_eq!(config.fetcher.timeout, 30);
        // Proxy should be None by default (commented out in tarzi.toml)
        assert_eq!(config.fetcher.proxy, None);
        assert_eq!(config.search.mode, SEARCH_MODE_WEBQUERY);
        assert_eq!(config.search.engine, SEARCH_ENGINE_DUCKDUCKGO);
        assert_eq!(config.search.query_pattern, DEFAULT_QUERY_PATTERN);
        assert_eq!(config.search.limit, DEFAULT_SEARCH_LIMIT);
        assert_eq!(config.search.autoswitch, AUTOSWITCH_STRATEGY_SMART);
        // API keys should be None by default (commented out in tarzi.toml)
        assert_eq!(config.search.brave_api_key, None);
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

    #[test]
    fn test_config_loading_precedence() {
        use std::fs;
        use tempfile::tempdir;

        let temp_dir = tempdir().unwrap();
        let project_config_path = temp_dir.path().join("tarzi.toml");
        let user_config_path = temp_dir.path().join(".tarzi.toml");

        // Create project config
        let project_config_str = r#"
[general]
log_level = "debug"
timeout = 60

[fetcher]
mode = "browser_headless"
format = "markdown"
timeout = 30

[search]
engine = "bing"
mode = "webquery"
limit = 10
"#;
        fs::write(&project_config_path, project_config_str).unwrap();

        // Create user config (should override project config)
        let user_config_str = r#"
[general]
log_level = "warn"
timeout = 45

[fetcher]
mode = "plain_request"
format = "json"
timeout = 60

[search]
engine = "google"
mode = "apiquery"
limit = 5
brave_api_key = "user_brave_key"
exa_api_key = "user_exa_key"
travily_api_key = "user_travily_key"
"#;
        fs::write(&user_config_path, user_config_str).unwrap();

        // Temporarily change HOME to temp_dir for testing
        let original_home = std::env::var("HOME").ok();
        unsafe {
            std::env::set_var("HOME", temp_dir.path().to_str().unwrap());
        }

        // Test loading with precedence
        let config = Config::load_with_precedence().unwrap();

        // User config should take precedence over project config
        assert_eq!(config.general.log_level, "warn"); // from user config
        assert_eq!(config.general.timeout, 45); // from user config
        assert_eq!(config.fetcher.mode, FETCHER_MODE_PLAIN_REQUEST); // from user config
        assert_eq!(config.fetcher.format, FORMAT_JSON); // from user config
        assert_eq!(config.fetcher.timeout, 60); // from user config
        assert_eq!(config.search.engine, SEARCH_ENGINE_GOOGLE); // from user config
        assert_eq!(config.search.mode, SEARCH_MODE_APIQUERY); // from user config
        assert_eq!(config.search.limit, 5); // from user config
        assert_eq!(
            config.search.brave_api_key,
            Some("user_brave_key".to_string())
        ); // from user config
        assert_eq!(config.search.exa_api_key, Some("user_exa_key".to_string()));
        assert_eq!(
            config.search.travily_api_key,
            Some("user_travily_key".to_string())
        );

        // Restore original HOME
        if let Some(home) = original_home {
            unsafe {
                std::env::set_var("HOME", home);
            }
        } else {
            unsafe {
                std::env::remove_var("HOME");
            }
        }
    }

    #[test]
    fn test_cli_params_override() {
        let mut config = Config::new();

        // Set some default values
        config.fetcher.mode = FETCHER_MODE_BROWSER_HEADLESS.to_string();
        config.fetcher.format = FORMAT_MARKDOWN.to_string();
        config.search.mode = SEARCH_MODE_WEBQUERY.to_string();
        config.search.limit = DEFAULT_SEARCH_LIMIT;
        config.search.engine = SEARCH_ENGINE_BING.to_string();

        // Create CLI parameters
        let mut cli_params = CliConfigParams::new();
        cli_params.fetcher_format = Some(FORMAT_JSON.to_string());
        cli_params.search_limit = Some(DEFAULT_SEARCH_LIMIT);
        cli_params.search_engine = Some(SEARCH_ENGINE_GOOGLE.to_string());

        // Apply CLI parameters
        config.apply_cli_params(&cli_params);

        // CLI parameters should override config values
        assert_eq!(config.fetcher.mode, FETCHER_MODE_BROWSER_HEADLESS);
        assert_eq!(config.fetcher.format, FORMAT_JSON);
        assert_eq!(config.search.mode, SEARCH_MODE_WEBQUERY);
        assert_eq!(config.search.limit, DEFAULT_SEARCH_LIMIT);
        assert_eq!(config.search.engine, SEARCH_ENGINE_GOOGLE);
    }

    #[test]
    fn test_config_merge() {
        let mut base_config = Config::new();

        // Set some base values
        base_config.general.log_level = LOG_LEVEL_INFO.to_string();
        base_config.fetcher.mode = FETCHER_MODE_BROWSER_HEADLESS.to_string();
        base_config.search.engine = SEARCH_ENGINE_BING.to_string();

        let override_config = Config {
            general: GeneralConfig {
                log_level: LOG_LEVEL_DEBUG.to_string(),
                timeout: 60,
            },
            fetcher: FetcherConfig {
                mode: FETCHER_MODE_PLAIN_REQUEST.to_string(),
                format: FORMAT_JSON.to_string(),
                user_agent: "Custom Agent".to_string(),
                timeout: 45,
                proxy: Some("http://proxy:8080".to_string()),
                web_driver: CHROMEDRIVER.to_string(),
                web_driver_url: Some("http://localhost:4444".to_string()),
            },
            search: SearchConfig {
                mode: SEARCH_MODE_APIQUERY.to_string(),
                engine: SEARCH_ENGINE_GOOGLE.to_string(),
                query_pattern: "custom pattern".to_string(),
                limit: DEFAULT_SEARCH_LIMIT,
                autoswitch: AUTOSWITCH_STRATEGY_NONE.to_string(),
                brave_api_key: Some("test_key".to_string()),
                exa_api_key: Some("override_exa_key".to_string()),
                travily_api_key: Some("override_travily_key".to_string()),
                baidu_api_key: None,
            },
        };

        // Merge override config into base config
        base_config.merge(&override_config);

        // Override config values should take precedence
        assert_eq!(base_config.general.log_level, LOG_LEVEL_DEBUG);
        assert_eq!(base_config.general.timeout, 60);
        assert_eq!(base_config.fetcher.mode, FETCHER_MODE_PLAIN_REQUEST);
        assert_eq!(base_config.fetcher.format, FORMAT_JSON);
        assert_eq!(base_config.fetcher.user_agent, "Custom Agent");
        assert_eq!(base_config.fetcher.timeout, 45);
        assert_eq!(
            base_config.fetcher.proxy,
            Some("http://proxy:8080".to_string())
        );
        assert_eq!(base_config.fetcher.web_driver, CHROMEDRIVER);
        assert_eq!(
            base_config.fetcher.web_driver_url,
            Some("http://localhost:4444".to_string())
        );
        assert_eq!(base_config.search.mode, SEARCH_MODE_APIQUERY);
        assert_eq!(base_config.search.engine, SEARCH_ENGINE_GOOGLE);
        assert_eq!(base_config.search.query_pattern, "custom pattern");
        assert_eq!(base_config.search.limit, DEFAULT_SEARCH_LIMIT);
        assert_eq!(base_config.search.autoswitch, AUTOSWITCH_STRATEGY_NONE);
        assert_eq!(
            base_config.search.brave_api_key,
            Some("test_key".to_string())
        );
        assert_eq!(
            base_config.search.exa_api_key,
            Some("override_exa_key".to_string())
        );
        assert_eq!(
            base_config.search.travily_api_key,
            Some("override_travily_key".to_string())
        );
    }
}
