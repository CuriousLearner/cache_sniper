# CacheSniper - CDN & Cache Invalidation Tester 🛠️

CacheSniper is a Rust-based CLI tool to analyze CDN cache headers and detect cache invalidation behavior.

## 🚀 Features
- Checks HTTP headers like `Cache-Control`, `ETag`, `Last-Modified`, and `Expires`.
- Displays results in a tabular format with emojis.
- Supports colored output for better readability.

## 🔧 Installation
```sh
cargo build --release
```

## 🔥 Usage
```sh
cargo run -- --url "https://example.com"
```
