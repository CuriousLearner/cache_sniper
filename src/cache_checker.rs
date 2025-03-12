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
    cdn_provider: String,
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

    print_results(url, &cache_control, &etag, &last_modified, &expires);

    Ok(CacheResult {
        url: url.to_string(),
        cache_control,
        etag,
        last_modified,
        expires,
        cdn_provider,
    })
}
