use crate::constants::{WEBDRIVER_CHECK_TIMEOUT, WEBDRIVER_LEGACY_DEFAULT_URL};
use reqwest;
use tokio::time::timeout;

/// Check if WebDriver server is available at the default endpoint
pub async fn is_webdriver_available() -> bool {
    let webdriver_url = std::env::var("TARZI_WEBDRIVER_URL")
        .unwrap_or_else(|_| WEBDRIVER_LEGACY_DEFAULT_URL.to_string());

    // Try to connect to WebDriver with a short timeout
    match timeout(
        WEBDRIVER_CHECK_TIMEOUT,
        reqwest::get(&format!("{webdriver_url}/status")),
    )
    .await
    {
        Ok(Ok(response)) => response.status().is_success(),
        _ => false,
    }
}
