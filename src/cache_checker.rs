use crate::http_client::fetch_headers;
use crate::utils::print_results;
use std::error::Error;

/// Analyzes caching headers for a given URL.
pub async fn check_cache(url: &str) -> Result<(), Box<dyn Error>> {
    let headers = fetch_headers(url).await?;

    let cache_control = headers.get("cache-control").map(|v| v.to_str().unwrap_or("")).unwrap_or("None");
    let etag = headers.get("etag").map(|v| v.to_str().unwrap_or("")).unwrap_or("None");
    let last_modified = headers.get("last-modified").map(|v| v.to_str().unwrap_or("")).unwrap_or("None");
    let expires = headers.get("expires").map(|v| v.to_str().unwrap_or("")).unwrap_or("None");

    print_results(url, cache_control, etag, last_modified, expires);
    Ok(())
}
