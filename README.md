# 🚀 CacheSniper - CDN & Cache Invalidation Tester 🛠️

CacheSniper is a **Rust-based CLI tool** for testing **CDN caching behavior**.
It helps developers analyze **Cache-Control**, **ETag**, **Last-Modified**, and **Expires** headers
to determine whether a webpage is being cached properly and how cache invalidation works.

---

## 📌 **Features**

✅ **Parallel Scanning** - Analyze multiple URLs at the same time
✅ **JSON Output** - Export cache test results in JSON format
✅ **Save Results to File** - Use `--output` to store findings
✅ **CDN Detection** - Automatically identifies CDN provider
✅ **Prometheus Metrics** - Monitor CDN caching behavior over time
✅ **Verbose Mode** - Show full HTTP headers for deep debugging
✅ **Tabular & Colorized Output** - Easy-to-read terminal display

---

## 🔧 **Installation**

Ensure you have **Rust installed**:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then, clone and build:

```sh
git clone https://github.com/your-repo/cache_sniper.git
cd cache_sniper
cargo build --release
```

To install globally:

```sh
cargo install --path .
```

---

## 🔥 **Usage**

### **Basic Check**

Scan a single URL:

```sh
cache_sniper --url "https://example.com"
```

### **Scan Multiple URLs**

```sh
cache_sniper --urls "https://example1.com" "https://example2.com"
```

### **Enable JSON Output**

```sh
cache_sniper --url "https://example.com" --json
```

### **Save Results to a File**

```sh
cache_sniper --url "https://example.com" --json --output results.json
```

### **Start Prometheus Metrics Server**

Run a metrics server at `http://localhost:9090/metrics`:

```sh
cache_sniper --metrics
```

### **Enable Verbose Mode (See Full Headers)**

```sh
cache_sniper --url "https://example.com" --verbose
```

---

## 📊 **Example Output**

```
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
