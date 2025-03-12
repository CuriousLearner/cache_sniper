# 🚀 CacheSniper - CDN & Cache Invalidation Tester 🛠️

CacheSniper is a **Rust-based CLI tool** for testing **CDN caching behavior**.
It helps developers analyze **Cache-Control**, **ETag**, **Last-Modified**, and **Expires** headers
to determine whether a webpage is being cached properly and how cache invalidation works.

---

## 📌 **Features**
✅ **Parallel Scanning** - Analyze multiple URLs at the same time
✅ **JSON Output** - Export cache test results in JSON format
✅ **Prometheus Metrics** - Monitor CDN caching behavior over time
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

---

## 🔥 **Usage**
### **Basic Check**
Scan a single URL:
```sh
cargo run -- --urls "https://example.com"
```

### **Scan Multiple URLs**
```sh
cargo run -- --urls "https://example1.com" "https://example2.com"
```

### **Enable JSON Output**
```sh
cargo run -- --urls "https://example.com" --json
```

### **Start Prometheus Metrics Server**
Run a metrics server at `http://localhost:9090/metrics`:
```sh
cargo run -- --metrics
```

---

## 📊 **Example Output**
```
🌍 Scanning: https://example.com

┌───────────────┬───────────────────────────┐
│ 🔍 Header     │ 📜 Value                   │
├───────────────┼───────────────────────────┤
│ 🛠 Cache-Control │ max-age=3600               │
│ 🔄 ETag       │ "abc123"                   │
│ 📆 Last-Modified │ Mon, 10 Mar 2025 12:00:00 GMT │
│ ⏳ Expires    │ Tue, 11 Mar 2025 12:00:00 GMT │
└───────────────┴───────────────────────────┘

✅ Success: This page is being cached!
```

---

## 🎯 **Why is this project helpful?**
**CacheSniper** helps:
- **Developers** verify cache settings to improve performance
- **DevOps engineers** ensure proper CDN caching across services
- **Security teams** detect cache poisoning vulnerabilities
- **API teams** confirm responses respect caching policies

---
### **🚀 Future Enhancements**
- 🔄 Add **cache-busting tests** (send different ETags to check invalidation)
- 📈 Integrate **Grafana dashboards** with Prometheus metrics
- 🌍 Support **batch URL scanning from a file**
- 🔎 Detect **cache inconsistencies** across regions

---
## 📜 **License**
MIT License. Use freely and contribute!
