use reqwest;
use std::time::Duration;
use tokio::time::timeout;

/// Check if WebDriver server is available at the default endpoint
pub async fn is_webdriver_available() -> bool {
    let webdriver_url = std::env::var("TARZI_WEBDRIVER_URL")
        .unwrap_or_else(|_| "http://localhost:4444".to_string());

    // Try to connect to WebDriver with a short timeout
    match timeout(
        Duration::from_secs(2),
        reqwest::get(&format!("{}/status", webdriver_url)),
    )
    .await
    {
        Ok(Ok(response)) => response.status().is_success(),
        _ => false,
    }
}
