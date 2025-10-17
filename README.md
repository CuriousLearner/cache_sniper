# 🚀 CacheSniper - CDN & Cache Invalidation Tester 🛠️

CacheSniper is a **Rust-based CLI tool** for testing **CDN caching behavior**.
It helps developers analyze **Cache-Control**, **ETag**, **Last-Modified**, and **Expires** headers
to determine whether a webpage is being cached properly and how cache invalidation works.

---

## 📌 **Features**

* **Parallel Scanning** - Analyze multiple URLs at the same time
* **Cache Validation (`--validate`)** - Ensures cache consistency
* **CDN-Specific Analysis** - Parse Cloudflare (CF-Cache-Status), Fastly (X-Cache), and generic CDN headers
* **Smart Cache Verdicts** - Interprets cache status (HIT/MISS/DYNAMIC/BYPASS) with explanations
* **CI/CD Integration** - `--exit-code` flag returns exit code 1 when caching fails (perfect for pipelines)
* **JSON Output** - Export cache test results in JSON format with all CDN headers
* **Save Results to File** - Use `--output` to store findings
* **CDN Detection** - Automatically identifies 23+ CDN providers
* **Prometheus Metrics** - Monitor CDN caching behavior over time (requires Redis)
* **Comprehensive Tests** - 8+ unit tests covering CDN detection and cache validation
* **Tabular & Colorized Output** - Easy-to-read terminal display

---

## 🔧 **Installation**

Ensure you have **Rust installed**:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then, clone and build:

```bash
git clone https://github.com/CuriousLearner/cache_sniper.git
cd cache_sniper
cargo build --release
```

To install globally:

```bash
cargo install --path .
```

**Optional:** Redis is only required if you want to use the `--metrics` flag for Prometheus monitoring. For basic cache checking, Redis is not needed.

---

## 🔥 **Usage**

### **Basic Check**

Scan a single URL:

```bash
cache_sniper --url "https://example.com"
```

### **Scan Multiple URLs**

```bash
cache_sniper --urls "https://example1.com" "https://example2.com"
```

### **Enable JSON Output**

```bash
cache_sniper --url "https://example.com" --json
```

### **Save Results to a File**

```bash
cache_sniper --url "https://example.com" --json --output results.json
```

### **Start Prometheus Metrics Server**

Run a metrics server at `http://localhost:9090/metrics`:

```bash
cache_sniper --metrics
```

**Note:** Metrics mode requires Redis to be running on `localhost:6379`. Install and start Redis:

```bash
# macOS
brew install redis
brew services start redis

# Ubuntu/Debian
sudo apt-get install redis-server
sudo systemctl start redis

# Other systems - see https://redis.io/download
```

### **Validate Cache Behavior (`--validate`)**

Check if caching is **working correctly** by sending a normal request and a `Cache-Control: no-cache` request, then comparing responses:

```bash
cache_sniper --url "https://example.com" --validate
```

**How it works:**

- If `ETag`, `Last-Modified`, and response **do not change**, cache is working correctly. ✅
- If **values change**, the cache **might not be working consistently**. ⚠️
- If **no cache headers exist**, validation is skipped automatically. 🚨

### **CI/CD Integration (`--exit-code`)**

Use in CI pipelines to fail builds when caching is not properly configured:

```bash
# Fails with exit code 1 if caching is disabled or broken
cache_sniper --url "https://example.com" --exit-code

# Perfect for GitHub Actions, GitLab CI, CircleCI, etc.
cache_sniper --url "https://api.example.com" --validate --exit-code --json
```

**Use cases:**
- Verify CDN configuration after deployment
- Ensure static assets are cacheable before merging PRs
- Monitor cache behavior in staging environments

---

## 🔍 **CDN-Specific Features**

CacheSniper understands CDN-specific headers and provides intelligent verdicts:

### **Cloudflare**
- **CF-Cache-Status**: HIT, MISS, EXPIRED, DYNAMIC, BYPASS, REVALIDATED
- **CF-Ray**: Request ID for debugging
- **Example verdict**: `"HIT - Served from Cloudflare cache"`

### **Fastly / Akamai / Generic CDNs**
- **X-Cache**: HIT, MISS
- **X-Served-By**: Cache server identifier
- **X-Cache-Hits**: Number of cache hits
- **Age**: Cache age in seconds

### **Example JSON Output**

```json
{
  "url": "https://www.rust-lang.org",
  "is_cached": true,
  "cache_verdict": "HIT - Served from github cache",
  "cdn_provider": "github",
  "x_cache": "HIT",
  "x_cache_hits": "1",
  "age": "16",
  "cache_control": "max-age=600",
  "etag": "\"68f16b7d-4899\""
}
```

---

## 📊 **Example Output**

```bash
🌍 Scanning: https://example.com
🌐 CDN Provider: Cloudflare

┌──────────────────┬───────────────────────────────┐
│ 🔍 Header        ┆ 📜 Value                      │
╞══════════════════╪═══════════════════════════════╡
│ 🛠 Cache-Control  ┆ max-age=86400                 │
├──────────────────┼───────────────────────────────┤
│ 🔄 ETag          ┆ "abc123"                      │
├──────────────────┼───────────────────────────────┤
│ 📆 Last-Modified ┆ Tue, 11 Mar 2025 06:28:07 GMT │
├──────────────────┼───────────────────────────────┤
│ ⏳ Expires       ┆ None                          │
└──────────────────┴───────────────────────────────┘

✅ Success: This page is being cached!
🔍 CF-Cache-Status: HIT
⏱️  Age: 120 seconds

📊 Verdict: HIT - Served from Cloudflare cache
```

### **Example Output for `--validate`**

```bash
🌍 Validating Cache for: https://example.com
🔄 ETag Before: "abc123"
🔄 ETag After : "abc123"
📆 Last-Modified Before: Tue, 11 Mar 2025 06:28:07 GMT
📆 Last-Modified After : Tue, 11 Mar 2025 06:28:07 GMT
⏳ Age Header: 86400

✅ Success: Cache is working correctly!
```

If no cache headers are detected:

```bash
🚨 Warning: No caching detected on http://localhost:9001, skipping validation!
```

---

## 🎯 **Why is this project helpful?**

**CacheSniper** helps:

- **DevOps engineers** debug caching behavior and identify stale content issues.
- **SREs** ensure cache invalidation works in **CI/CD pipelines**.
- **Security teams** detect **cache poisoning vulnerabilities**.
- **Monitoring teams** track **real-time cache behavior** via Prometheus.

---

## 📜 **License**

MIT License. Use freely and contribute!
