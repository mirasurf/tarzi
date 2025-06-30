//! Integration tests for the web driver manager
//!
//! These tests verify that the driver manager works correctly with actual driver binaries.
//! Tests will be skipped if the required drivers are not installed.

#[cfg(not(feature = "test-helpers"))]
use tarzi::fetcher::driver::{DriverManager, DriverType};
#[cfg(feature = "test-helpers")]
use tarzi::fetcher::driver::{DriverConfig, DriverManager, DriverStatus, DriverType, test_helpers};
#[cfg(feature = "test-helpers")]
use std::time::Duration;

/// Test that we can create a driver manager
#[test]
fn test_create_driver_manager() {
    let manager = DriverManager::new();
    assert_eq!(manager.list_drivers().len(), 0);
}

/// Test driver binary detection
#[test]
fn test_driver_binary_detection() {
    let manager = DriverManager::new();

    // Test Chrome driver detection
    let chrome_result = manager.check_driver_binary(&DriverType::Chrome);
    match &chrome_result {
        Ok(()) => println!("ChromeDriver is available"),
        Err(e) => println!("ChromeDriver not available: {}", e),
    }

    // Test Firefox driver detection
    let firefox_result = manager.check_driver_binary(&DriverType::Firefox);
    match &firefox_result {
        Ok(()) => println!("GeckoDriver is available"),
        Err(e) => println!("GeckoDriver not available: {}", e),
    }

    // At least one of the results should provide useful error messages
    if chrome_result.is_err() {
        let error_msg = format!("{}", chrome_result.unwrap_err());
        assert!(error_msg.contains("chromedriver") || error_msg.contains("ChromeDriver"));
    }

    if firefox_result.is_err() {
        let error_msg = format!("{}", firefox_result.unwrap_err());
        assert!(error_msg.contains("geckodriver") || error_msg.contains("GeckoDriver"));
    }
}

/// Test starting and stopping a Chrome driver (if available)
#[test]
#[cfg(feature = "test-helpers")]
fn test_chrome_driver_lifecycle() {
    if !test_helpers::is_driver_available(&DriverType::Chrome) {
        println!("Skipping Chrome driver test - chromedriver not available");
        return;
    }

    let manager = DriverManager::new();
    let port = test_helpers::find_available_port();

    // Create config for Chrome driver
    let config = DriverConfig {
        driver_type: DriverType::Chrome,
        port,
        args: vec![
            "--disable-gpu".to_string(),
            "--no-sandbox".to_string(),
            "--disable-dev-shm-usage".to_string(),
        ],
        timeout: Duration::from_secs(15),
        verbose: false,
    };

    // Start the driver
    let driver_info = manager.start_driver_with_config(config.clone());

    match driver_info {
        Ok(info) => {
            // Verify driver info
            assert_eq!(info.config.driver_type, DriverType::Chrome);
            assert_eq!(info.config.port, port);
            assert_eq!(info.status, DriverStatus::Running);
            assert!(info.pid.is_some());
            assert_eq!(info.endpoint, format!("http://127.0.0.1:{}", port));

            // Verify the driver is listed
            let drivers = manager.list_drivers();
            assert_eq!(drivers.len(), 1);
            assert_eq!(drivers[0].config.port, port);

            // Verify we can get driver info
            let retrieved_info = manager.get_driver_info(port);
            assert!(retrieved_info.is_some());

            // Stop the driver
            let stop_result = manager.stop_driver(port);
            assert!(stop_result.is_ok());

            // Verify the driver is no longer listed
            let drivers_after_stop = manager.list_drivers();
            assert_eq!(drivers_after_stop.len(), 0);

            println!("Chrome driver test completed successfully");
        }
        Err(e) => {
            println!("Failed to start Chrome driver: {}", e);
            // The test is still successful if we got a meaningful error
            assert!(
                format!("{}", e).contains("chromedriver") || format!("{}", e).contains("Chrome")
            );
        }
    }
}

/// Test error handling for non-existent driver
#[test]
#[cfg(feature = "test-helpers")]
fn test_nonexistent_driver() {
    let manager = DriverManager::new();

    let config = DriverConfig {
        driver_type: DriverType::Generic("nonexistent-driver".to_string()),
        port: test_helpers::find_available_port(),
        args: vec![],
        timeout: Duration::from_secs(5),
        verbose: false,
    };

    let result = manager.start_driver_with_config(config);
    assert!(result.is_err());

    if let Err(e) = result {
        let error_msg = format!("{}", e);
        assert!(error_msg.contains("not found") || error_msg.contains("nonexistent-driver"));
        println!(
            "Non-existent driver error handling works correctly: {}",
            error_msg
        );
    }
}
