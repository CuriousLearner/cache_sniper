use reqwest::header::{HeaderMap, USER_AGENT};
use std::error::Error;

/// Fetches headers from the target URL.
pub async fn fetch_headers(url: &str) -> Result<HeaderMap, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client.get(url)
        .header(USER_AGENT, "CacheSniper/1.0")
        .send()
        .await?;
    
    Ok(response.headers().clone())
}
