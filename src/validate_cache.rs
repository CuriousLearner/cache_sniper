use reqwest::header::{CACHE_CONTROL, ETAG, LAST_MODIFIED, AGE};
use std::error::Error;
use serde::Serialize;
use colored::*;

#[derive(Serialize)]
pub struct CacheValidationResult {
    url: String,
    etag_before: String,
    etag_after: String,
    last_modified_before: String,
    last_modified_after: String,
    age: String,
    is_consistent: bool,
}

/// Validates cache consistency by comparing responses before and after `Cache-Control: no-cache`
pub async fn validate_cache(url: &str) -> Result<CacheValidationResult, Box<dyn Error + Send + Sync>> {
    let client = reqwest::Client::new();

    // First Request (Normal request)
    let res1 = client.get(url).send().await?;
    let headers1 = res1.headers().clone();
    let body1 = res1.text().await?;

    // Extract headers
    let cache_control = headers1.get(CACHE_CONTROL).map(|v| v.to_str().unwrap_or("")).unwrap_or("");
    let etag_before = headers1.get(ETAG).map(|v| v.to_str().unwrap_or("")).unwrap_or("");
    let last_modified_before = headers1.get(LAST_MODIFIED).map(|v| v.to_str().unwrap_or("")).unwrap_or("");
    let age_header = headers1.get(AGE).map(|v| v.to_str().unwrap_or("")).unwrap_or("");

    // 🚨 If no cache-related headers exist, skip validation
    if cache_control.is_empty() && etag_before.is_empty() && last_modified_before.is_empty() {
        println!("\n🚨 {} No caching detected on {}, skipping validation!", "Warning:".red().bold(), url);
        return Err("No caching detected".into());
    }

    // Second Request (Force refresh with `Cache-Control: no-cache`)
    let res2 = client.get(url)
        .header(CACHE_CONTROL, "no-cache")
        .send().await?;

    let headers2 = res2.headers().clone();
    let body2 = res2.text().await?;

    let etag_after = headers2.get(ETAG).map(|v| v.to_str().unwrap_or("")).unwrap_or("").to_string();
    let last_modified_after = headers2.get(LAST_MODIFIED).map(|v| v.to_str().unwrap_or("")).unwrap_or("").to_string();

    // Compare responses
    let is_consistent = etag_before == etag_after && last_modified_before == last_modified_after && body1 == body2;

    // Print results
    println!("\n🌍 Validating Cache for: {}", url);
    println!("🔄 ETag Before: {}", etag_before);
    println!("🔄 ETag After : {}", etag_after);
    println!("📆 Last-Modified Before: {}", last_modified_before);
    println!("📆 Last-Modified After : {}", last_modified_after);
    println!("⏳ Age Header: {}", age_header);

    if is_consistent {
        println!("✅ {} Cache is working correctly!", "Success:".green().bold());
    } else {
        println!("🚨 {} Cache behavior is inconsistent!", "Warning:".red().bold());
    }

    Ok(CacheValidationResult {
        url: url.to_string(),
        etag_before: etag_before.to_string(),
        etag_after,
        last_modified_before: last_modified_before.to_string(),
        last_modified_after,
        age: age_header.to_string(),
        is_consistent,
    })
}
