//! Unit tests for web driver types management
//!
//! Tests verify that the two driver types (external and self-managed) are mutually exclusive:
//! 1. External: configured by web_driver_url - if set, use it exclusively and fail if unavailable
//! 2. Self-managed: managed by DriverManager - used only if web_driver_url is not set

use tarzi::{
    config::Config,
    fetcher::browser::BrowserManager,
};

/// Helper function to create a config with external web driver URL
fn create_config_with_external_url(url: &str) -> Config {
    let mut config = Config::default();
    config.fetcher.web_driver_url = Some(url.to_string());
    config.fetcher.web_driver = "geckodriver".to_string();
    config
}

/// Helper function to create a config without external URL (for self-managed)
fn create_config_for_self_managed(driver_type: &str) -> Config {
    let mut config = Config::default();
    config.fetcher.web_driver_url = None;
    config.fetcher.web_driver = driver_type.to_string();
    config
}

/// Helper function to create a config with empty external URL (should use self-managed)
fn create_config_with_empty_url() -> Config {
    let mut config = Config::default();
    config.fetcher.web_driver_url = Some("".to_string());
    config.fetcher.web_driver = "geckodriver".to_string();
    config
}

#[tokio::test]
async fn test_external_driver_type_with_valid_url() {
    // This test requires a running WebDriver server at localhost:4444
    // If no server is running, it should fail appropriately
    let config = create_config_with_external_url("http://localhost:4444");
    let mut browser_manager = BrowserManager::from_config(&config);

    // Try to get browser - this will internally test external type
    let result = browser_manager.get_or_create_browser(true).await;
    
    match result {
        Ok(_browser) => {
            // If successful, it should have used the external URL (no managed driver)
            assert!(!browser_manager.has_managed_driver());
            println!("External driver type test passed - WebDriver found at configured URL");
        }
        Err(e) => {
            // If failed, it should be a specific external driver error
            let error_msg = format!("{}", e);
            assert!(error_msg.contains("External WebDriver server is not available"));
            assert!(error_msg.contains("http://localhost:4444"));
            assert!(error_msg.contains("Please ensure the WebDriver server is running"));
            println!("External driver type test passed - Correct error for unavailable external URL");
        }
    }
}

#[tokio::test]
async fn test_external_driver_type_with_invalid_url() {
    let config = create_config_with_external_url("http://localhost:9999");
    let mut browser_manager = BrowserManager::from_config(&config);

    let result = browser_manager.get_or_create_browser(true).await;
    
    // Should always fail with external driver error for invalid URL
    assert!(result.is_err());
    let error_msg = format!("{}", result.unwrap_err());
    assert!(error_msg.contains("External WebDriver server is not available"));
    assert!(error_msg.contains("http://localhost:9999"));
    assert!(error_msg.contains("Please ensure the WebDriver server is running"));
    println!("External driver type with invalid URL test passed");
}

#[tokio::test]
async fn test_self_managed_driver_type_firefox() {
    let config = create_config_for_self_managed("geckodriver");
    let mut browser_manager = BrowserManager::from_config(&config);

    let result = browser_manager.get_or_create_browser(true).await;
    
    match result {
        Ok(_browser) => {
            // Should have started a managed driver
            assert!(browser_manager.has_managed_driver());
            
            // Cleanup
            let _ = browser_manager.cleanup_managed_driver().await;
            println!("Self-managed Firefox driver test passed");
        }
        Err(e) => {
            // Should be a self-managed driver error, not external
            let error_msg = format!("{}", e);
            assert!(error_msg.contains("No self-managed WebDriver could be started"));
            assert!(!error_msg.contains("External WebDriver"));
            println!("Self-managed Firefox driver test passed - Correct error for unavailable drivers: {}", error_msg);
        }
    }
}

#[tokio::test]
async fn test_self_managed_driver_type_chrome() {
    let config = create_config_for_self_managed("chromedriver");
    let mut browser_manager = BrowserManager::from_config(&config);

    let result = browser_manager.get_or_create_browser(true).await;
    
    match result {
        Ok(_browser) => {
            // Should have started a managed driver
            assert!(browser_manager.has_managed_driver());
            
            // Cleanup
            let _ = browser_manager.cleanup_managed_driver().await;
            println!("Self-managed Chrome driver test passed");
        }
        Err(e) => {
            // Should be a self-managed driver error, not external
            let error_msg = format!("{}", e);
            assert!(error_msg.contains("No self-managed WebDriver could be started"));
            assert!(!error_msg.contains("External WebDriver"));
            println!("Self-managed Chrome driver test passed - Correct error for unavailable drivers: {}", error_msg);
        }
    }
}

#[tokio::test]
async fn test_empty_external_url_uses_self_managed() {
    let config = create_config_with_empty_url();
    let mut browser_manager = BrowserManager::from_config(&config);

    let result = browser_manager.get_or_create_browser(true).await;
    
    match result {
        Ok(_browser) => {
            // Should use self-managed driver, not external - might have managed driver
            println!("Empty external URL test passed - correctly used self-managed driver");
            
            // Cleanup if managed driver was started
            let _ = browser_manager.cleanup_managed_driver().await;
        }
        Err(e) => {
            // Should be a self-managed driver error, not external
            let error_msg = format!("{}", e);
            assert!(error_msg.contains("No self-managed WebDriver could be started"));
            assert!(!error_msg.contains("External WebDriver"));
            println!("Empty external URL test passed - Correct self-managed error");
        }
    }
}

#[tokio::test]
async fn test_no_config_uses_self_managed() {
    let mut browser_manager = BrowserManager::new();

    let result = browser_manager.get_or_create_browser(true).await;
    
    match result {
        Ok(_browser) => {
            // Should use self-managed driver with default settings
            println!("No config test passed - correctly used self-managed driver");
            
            // Cleanup
            let _ = browser_manager.cleanup_managed_driver().await;
        }
        Err(e) => {
            // Should be a self-managed driver error
            let error_msg = format!("{}", e);
            assert!(error_msg.contains("No self-managed WebDriver could be started"));
            assert!(!error_msg.contains("External WebDriver"));
            println!("No config test passed - Correct self-managed error");
        }
    }
}

#[tokio::test]
async fn test_driver_types_are_exclusive() {
    // Test that external and self-managed are truly exclusive
    
    // First test: external URL configured should never use self-managed
    let config_external = create_config_with_external_url("http://localhost:9999");
    let mut browser_manager_external = BrowserManager::from_config(&config_external);
    
    let result_external = browser_manager_external.get_or_create_browser(true).await;
    assert!(result_external.is_err());
    assert!(!browser_manager_external.has_managed_driver()); // Should not have started a managed driver
    
    // Second test: no external URL should never try external URLs
    let config_self_managed = create_config_for_self_managed("geckodriver");
    let mut browser_manager_self = BrowserManager::from_config(&config_self_managed);
    
    let result_self = browser_manager_self.get_or_create_browser(true).await;
    match result_self {
        Ok(_) => {
            // If successful, must be self-managed
            assert!(browser_manager_self.has_managed_driver());
            let _ = browser_manager_self.cleanup_managed_driver().await;
        }
        Err(e) => {
            // If failed, should be self-managed error
            let error_msg = format!("{}", e);
            assert!(error_msg.contains("No self-managed WebDriver could be started"));
        }
    }
    
    println!("Driver types exclusivity test passed");
}

#[tokio::test]
async fn test_managed_driver_info() {
    let config = create_config_for_self_managed("geckodriver");
    let mut browser_manager = BrowserManager::from_config(&config);

    // Initially no managed driver
    assert!(!browser_manager.has_managed_driver());
    assert!(browser_manager.get_managed_driver_info().is_none());

    let result = browser_manager.get_or_create_browser(true).await;
    
    if result.is_ok() {
        // Should now have managed driver info
        assert!(browser_manager.has_managed_driver());
        let driver_info = browser_manager.get_managed_driver_info();
        assert!(driver_info.is_some());
        
        if let Some(info) = driver_info {
            assert!(info.endpoint.starts_with("http://"));
            assert!(info.pid.is_some());
        }
        
        // Cleanup should remove managed driver info
        let cleanup_result = browser_manager.cleanup_managed_driver().await;
        assert!(cleanup_result.is_ok());
        assert!(!browser_manager.has_managed_driver());
        assert!(browser_manager.get_managed_driver_info().is_none());
        
        println!("Managed driver info test passed");
    } else {
        println!("Managed driver info test skipped - no drivers available");
    }
}

#[tokio::test]
async fn test_multiple_calls_same_external_url() {
    let config = create_config_with_external_url("http://localhost:8888");
    let mut browser_manager = BrowserManager::from_config(&config);

    // Multiple calls should consistently return the same error for unavailable external URL
    for i in 0..3 {
        let result = browser_manager.get_or_create_browser(true).await;
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("External WebDriver server is not available"));
        assert!(error_msg.contains("http://localhost:8888"));
        
        // Should never have started a managed driver
        assert!(!browser_manager.has_managed_driver());
    }
    
    println!("Multiple external URL calls test passed");
}

#[tokio::test]
async fn test_config_merge_preserves_driver_type() {
    // Test that when configs are merged, driver type behavior is preserved
    let mut base_config = Config::default();
    base_config.fetcher.web_driver = "chromedriver".to_string();
    base_config.fetcher.web_driver_url = None;
    
    let mut override_config = Config::default();
    override_config.fetcher.web_driver_url = Some("http://localhost:7777".to_string());
    
    // Merge configs
    base_config.merge(override_config);
    
    // Should now use external driver type
    let mut browser_manager = BrowserManager::from_config(&base_config);
    let result = browser_manager.get_or_create_browser(true).await;
    
    assert!(result.is_err());
    let error_msg = format!("{}", result.unwrap_err());
    assert!(error_msg.contains("External WebDriver server is not available"));
    assert!(error_msg.contains("http://localhost:7777"));
    
    println!("Config merge driver type test passed");
}

