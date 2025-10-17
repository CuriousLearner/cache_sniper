use crate::http_client::fetch_headers;
use crate::utils::print_results;
use serde::Serialize;
use std::error::Error;
use std::fmt;

#[derive(Serialize)]
pub struct CacheResult {
    pub url: String,
    pub cache_control: String,
    pub etag: String,
    pub last_modified: String,
    pub expires: String,
    pub cdn_provider: String,
    // CDN-specific headers
    pub cf_cache_status: Option<String>,  // Cloudflare
    pub cf_ray: Option<String>,           // Cloudflare request ID
    pub x_cache: Option<String>,          // Fastly, Akamai, others
    pub x_served_by: Option<String>,      // Fastly
    pub age: Option<String>,              // Generic cache age
    pub x_cache_hits: Option<String>,     // Cache hit count
    // Cache verdict
    pub is_cached: bool,
    pub cache_verdict: String,
}

/// Custom error wrapper for consistent error handling
#[derive(Debug)]
struct CacheSniperError(String);

impl fmt::Display for CacheSniperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CacheSniperError {}

/// Determines cache verdict based on CDN-specific headers and standard cache headers
fn determine_cache_verdict(
    cf_cache_status: &Option<String>,
    x_cache: &Option<String>,
    cache_control: &str,
    etag: &str,
    cdn_provider: &str,
) -> (bool, String) {
    // Cloudflare-specific logic
    if let Some(cf_status) = cf_cache_status {
        let status_lower = cf_status.to_lowercase();
        return match status_lower.as_str() {
            "hit" => (true, "HIT - Served from Cloudflare cache".to_string()),
            "miss" => (false, "MISS - Origin fetch, not cached yet".to_string()),
            "expired" => (false, "EXPIRED - Cache entry stale, revalidating".to_string()),
            "stale" => (true, "STALE - Serving stale content while revalidating".to_string()),
            "bypass" => (false, "BYPASS - Cache rules bypassed".to_string()),
            "revalidated" => (true, "REVALIDATED - Cache validated with origin".to_string()),
            "updating" => (true, "UPDATING - Serving cached while updating".to_string()),
            "dynamic" => (false, "DYNAMIC - Content marked as uncacheable".to_string()),
            _ => (false, format!("Unknown Cloudflare status: {}", cf_status)),
        };
    }

    // Fastly/generic X-Cache logic
    if let Some(x_cache_val) = x_cache {
        let cache_lower = x_cache_val.to_lowercase();
        if cache_lower.contains("hit") {
            return (true, format!("HIT - Served from {} cache", cdn_provider));
        } else if cache_lower.contains("miss") {
            return (false, format!("MISS - Not in {} cache", cdn_provider));
        }
    }

    // Fallback to standard cache header analysis
    if cache_control != "None" && !cache_control.contains("no-cache") && !cache_control.contains("no-store") {
        if etag != "None" || cache_control.contains("max-age") {
            return (true, "Cacheable - Has cache headers".to_string());
        }
    }

    (false, "Not cached - No cache indicators".to_string())
}

/// Detects known CDNs based on the "Server" header
/// Detects known CDNs based on various headers dynamically
fn detect_cdn(headers: &reqwest::header::HeaderMap) -> String {
    let known_cdns = vec![
        "cloudflare", "fastly", "akamai", "cloudfront", "gcore", "bunnycdn",
        "cdn77", "stackpath", "edgecast", "limelight", "quic.cloud", "github",
        "imperva", "sucuri", "cachefly", "jsdelivr", "bootstrapcdn", "keycdn",
        "maxcdn", "belugacdn", "arvancloud", "google", "stackpathdns"
    ];

    let server_header = headers.get("server")
        .map(|v| v.to_str().unwrap_or(""))
        .unwrap_or("")
        .to_lowercase();

    let via_header = headers.get("via")
        .map(|v| v.to_str().unwrap_or(""))
        .unwrap_or("")
        .to_lowercase();

    let cdn_provider_header = headers.get("x-cdn-provider")
        .map(|v| v.to_str().unwrap_or(""))
        .unwrap_or("")
        .to_lowercase();

    // Check if any of the headers contain a known CDN keyword
    for cdn in &known_cdns {
        if server_header.contains(cdn) || via_header.contains(cdn) || cdn_provider_header.contains(cdn) {
            return cdn.to_string();
        }
    }

    // If no CDN is detected, return "Unknown" instead of "None"
    "Unknown".to_string()
}

pub async fn check_cache(url: &str, verbose: bool) -> Result<CacheResult, Box<dyn Error + Send + Sync>> {
    let headers = fetch_headers(url)
        .await
        .map_err(|e| Box::new(CacheSniperError(e.to_string())) as Box<dyn Error + Send + Sync>)?;

    if verbose {
        println!("
📜 Full Headers for {}:", url);
        for (key, value) in headers.iter() {
            println!("{}: {}", key, value.to_str().unwrap_or("INVALID UTF-8"));
        }
    }

    let cdn_provider = detect_cdn(&headers);

    if cdn_provider != "None" {
        println!("\n🌐 CDN Provider: {}", cdn_provider);
    }

    let cache_control = headers.get("cache-control").map(|v| v.to_str().unwrap_or("")).unwrap_or("None").to_string();
    let etag = headers.get("etag").map(|v| v.to_str().unwrap_or("")).unwrap_or("None").to_string();
    let last_modified = headers.get("last-modified").map(|v| v.to_str().unwrap_or("")).unwrap_or("None").to_string();
    let expires = headers.get("expires").map(|v| v.to_str().unwrap_or("")).unwrap_or("None").to_string();

    // Extract CDN-specific headers
    let cf_cache_status = headers.get("cf-cache-status").map(|v| v.to_str().unwrap_or("").to_string());
    let cf_ray = headers.get("cf-ray").map(|v| v.to_str().unwrap_or("").to_string());
    let x_cache = headers.get("x-cache").map(|v| v.to_str().unwrap_or("").to_string());
    let x_served_by = headers.get("x-served-by").map(|v| v.to_str().unwrap_or("").to_string());
    let age = headers.get("age").map(|v| v.to_str().unwrap_or("").to_string());
    let x_cache_hits = headers.get("x-cache-hits").map(|v| v.to_str().unwrap_or("").to_string());

    // Determine cache verdict
    let (is_cached, cache_verdict) = determine_cache_verdict(
        &cf_cache_status,
        &x_cache,
        &cache_control,
        &etag,
        &cdn_provider,
    );

    print_results(url, &cache_control, &etag, &last_modified, &expires);

    // Print CDN-specific info if available
    if let Some(ref status) = cf_cache_status {
        println!("🔍 CF-Cache-Status: {}", status);
    }
    if let Some(ref cache) = x_cache {
        println!("🔍 X-Cache: {}", cache);
    }
    if let Some(ref age_val) = age {
        println!("⏱️  Age: {} seconds", age_val);
    }

    println!("\n📊 Verdict: {}", cache_verdict);

    Ok(CacheResult {
        url: url.to_string(),
        cache_control,
        etag,
        last_modified,
        expires,
        cdn_provider,
        cf_cache_status,
        cf_ray,
        x_cache,
        x_served_by,
        age,
        x_cache_hits,
        is_cached,
        cache_verdict,
    })
}
