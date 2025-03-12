use crate::http_client::fetch_headers;
use crate::utils::print_results;
use serde::Serialize;
use std::error::Error;
use std::fmt;

#[derive(Serialize)]
pub struct CacheResult {
    url: String,
    cache_control: String,
    etag: String,
    last_modified: String,
    expires: String,
}

/// Custom error wrapper to ensure Send + Sync compatibility
#[derive(Debug)]
struct CacheSniperError(String);

impl fmt::Display for CacheSniperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CacheSniperError {}

/// Analyzes caching headers for a given URL.
pub async fn check_cache(url: &str) -> Result<CacheResult, Box<dyn Error + Send + Sync>> {
    let headers = fetch_headers(url)
        .await
        .map_err(|e| Box::new(CacheSniperError(e.to_string())) as Box<dyn Error + Send + Sync>)?;

    let cache_control = headers.get("cache-control").map(|v| v.to_str().unwrap_or("")).unwrap_or("None").to_string();
    let etag = headers.get("etag").map(|v| v.to_str().unwrap_or("")).unwrap_or("None").to_string();
    let last_modified = headers.get("last-modified").map(|v| v.to_str().unwrap_or("")).unwrap_or("None").to_string();
    let expires = headers.get("expires").map(|v| v.to_str().unwrap_or("")).unwrap_or("None").to_string();

    print_results(url, &cache_control, &etag, &last_modified, &expires);

    Ok(CacheResult {
        url: url.to_string(),
        cache_control,
        etag,
        last_modified,
        expires,
    })
}
