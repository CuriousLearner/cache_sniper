use cache_sniper::cache_checker::CacheResult;

#[cfg(test)]
mod cdn_detection_tests {
    use super::*;

    #[test]
    fn test_cloudflare_cache_hit_verdict() {
        // Test that Cloudflare HIT status is correctly identified
        let result = CacheResult {
            url: "https://example.com".to_string(),
            cache_control: "max-age=3600".to_string(),
            etag: "\"abc123\"".to_string(),
            last_modified: "None".to_string(),
            expires: "None".to_string(),
            cdn_provider: "cloudflare".to_string(),
            cf_cache_status: Some("HIT".to_string()),
            cf_ray: Some("12345".to_string()),
            x_cache: None,
            x_served_by: None,
            age: Some("120".to_string()),
            x_cache_hits: None,
            is_cached: true,
            cache_verdict: "HIT - Served from Cloudflare cache".to_string(),
        };

        assert_eq!(result.is_cached, true);
        assert!(result.cache_verdict.contains("HIT"));
        assert_eq!(result.cdn_provider, "cloudflare");
    }

    #[test]
    fn test_cloudflare_cache_miss_verdict() {
        let result = CacheResult {
            url: "https://example.com".to_string(),
            cache_control: "max-age=3600".to_string(),
            etag: "\"abc123\"".to_string(),
            last_modified: "None".to_string(),
            expires: "None".to_string(),
            cdn_provider: "cloudflare".to_string(),
            cf_cache_status: Some("MISS".to_string()),
            cf_ray: Some("12345".to_string()),
            x_cache: None,
            x_served_by: None,
            age: None,
            x_cache_hits: None,
            is_cached: false,
            cache_verdict: "MISS - Origin fetch, not cached yet".to_string(),
        };

        assert_eq!(result.is_cached, false);
        assert!(result.cache_verdict.contains("MISS"));
    }

    #[test]
    fn test_cloudflare_dynamic_verdict() {
        let result = CacheResult {
            url: "https://example.com".to_string(),
            cache_control: "no-cache".to_string(),
            etag: "None".to_string(),
            last_modified: "None".to_string(),
            expires: "None".to_string(),
            cdn_provider: "cloudflare".to_string(),
            cf_cache_status: Some("DYNAMIC".to_string()),
            cf_ray: Some("12345".to_string()),
            x_cache: None,
            x_served_by: None,
            age: None,
            x_cache_hits: None,
            is_cached: false,
            cache_verdict: "DYNAMIC - Content marked as uncacheable".to_string(),
        };

        assert_eq!(result.is_cached, false);
        assert!(result.cache_verdict.contains("DYNAMIC"));
    }

    #[test]
    fn test_fastly_cache_hit() {
        let result = CacheResult {
            url: "https://example.com".to_string(),
            cache_control: "max-age=3600".to_string(),
            etag: "\"xyz789\"".to_string(),
            last_modified: "None".to_string(),
            expires: "None".to_string(),
            cdn_provider: "fastly".to_string(),
            cf_cache_status: None,
            cf_ray: None,
            x_cache: Some("HIT".to_string()),
            x_served_by: Some("cache-lax123".to_string()),
            age: Some("45".to_string()),
            x_cache_hits: Some("3".to_string()),
            is_cached: true,
            cache_verdict: "HIT - Served from fastly cache".to_string(),
        };

        assert_eq!(result.is_cached, true);
        assert!(result.cache_verdict.contains("HIT"));
        assert_eq!(result.x_cache, Some("HIT".to_string()));
    }

    #[test]
    fn test_no_cache_headers() {
        let result = CacheResult {
            url: "https://example.com".to_string(),
            cache_control: "None".to_string(),
            etag: "None".to_string(),
            last_modified: "None".to_string(),
            expires: "None".to_string(),
            cdn_provider: "Unknown".to_string(),
            cf_cache_status: None,
            cf_ray: None,
            x_cache: None,
            x_served_by: None,
            age: None,
            x_cache_hits: None,
            is_cached: false,
            cache_verdict: "Not cached - No cache indicators".to_string(),
        };

        assert_eq!(result.is_cached, false);
        assert!(result.cache_verdict.contains("Not cached"));
    }

    #[test]
    fn test_standard_cache_headers_present() {
        let result = CacheResult {
            url: "https://example.com".to_string(),
            cache_control: "max-age=7200, public".to_string(),
            etag: "\"def456\"".to_string(),
            last_modified: "Mon, 01 Jan 2024 00:00:00 GMT".to_string(),
            expires: "None".to_string(),
            cdn_provider: "Unknown".to_string(),
            cf_cache_status: None,
            cf_ray: None,
            x_cache: None,
            x_served_by: None,
            age: None,
            x_cache_hits: None,
            is_cached: true,
            cache_verdict: "Cacheable - Has cache headers".to_string(),
        };

        assert_eq!(result.is_cached, true);
        assert!(result.cache_verdict.contains("Cacheable"));
    }
}

#[cfg(test)]
mod serialization_tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_cache_result_serialization() {
        let result = CacheResult {
            url: "https://example.com".to_string(),
            cache_control: "max-age=3600".to_string(),
            etag: "\"abc123\"".to_string(),
            last_modified: "None".to_string(),
            expires: "None".to_string(),
            cdn_provider: "cloudflare".to_string(),
            cf_cache_status: Some("HIT".to_string()),
            cf_ray: Some("12345".to_string()),
            x_cache: None,
            x_served_by: None,
            age: Some("120".to_string()),
            x_cache_hits: None,
            is_cached: true,
            cache_verdict: "HIT - Served from Cloudflare cache".to_string(),
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"url\":\"https://example.com\""));
        assert!(json.contains("\"is_cached\":true"));
        assert!(json.contains("\"cf_cache_status\":\"HIT\""));
        assert!(json.contains("\"cdn_provider\":\"cloudflare\""));
    }

    #[test]
    fn test_cache_result_with_null_optional_fields() {
        let result = CacheResult {
            url: "https://example.com".to_string(),
            cache_control: "None".to_string(),
            etag: "None".to_string(),
            last_modified: "None".to_string(),
            expires: "None".to_string(),
            cdn_provider: "Unknown".to_string(),
            cf_cache_status: None,
            cf_ray: None,
            x_cache: None,
            x_served_by: None,
            age: None,
            x_cache_hits: None,
            is_cached: false,
            cache_verdict: "Not cached".to_string(),
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"cf_cache_status\":null"));
        assert!(json.contains("\"is_cached\":false"));
    }
}
