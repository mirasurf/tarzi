use crate::constants::{CHROMEDRIVER_DEFAULT_URL, WEBDRIVER_CHECK_TIMEOUT};
use reqwest;
use tokio::time::timeout;

/// Check if WebDriver server is available at the default endpoint
pub async fn is_webdriver_available() -> bool {
    let webdriver_url = CHROMEDRIVER_DEFAULT_URL;

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
