use reqwest::{Client, header::HeaderMap};
use std::error::Error;

/// Fetches headers from the given URL
pub async fn fetch_headers(url: &str) -> Result<HeaderMap, Box<dyn Error + Send + Sync>> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    Ok(response.headers().clone())
}
