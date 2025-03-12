use comfy_table::{Table, Cell, Row};

/// Prints caching results in a formatted table
pub fn print_results(url: &str, cache_control: &str, etag: &str, last_modified: &str, expires: &str) {
    let mut table = Table::new();
    table.add_row(Row::from(vec![Cell::new("🔍 Header"), Cell::new("📜 Value")]));
    table.add_row(Row::from(vec![Cell::new("🛠 Cache-Control"), Cell::new(cache_control)]));
    table.add_row(Row::from(vec![Cell::new("🔄 ETag"), Cell::new(etag)]));
    table.add_row(Row::from(vec![Cell::new("📆 Last-Modified"), Cell::new(last_modified)]));
    table.add_row(Row::from(vec![Cell::new("⏳ Expires"), Cell::new(expires)]));

    println!("
🌍 Scanning: {}", url);
    println!("{}", table);
}
