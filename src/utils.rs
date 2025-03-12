use comfy_table::{Table, Cell, Row};
use colored::*;
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
    if cache_control == "None" && etag == "None" && last_modified == "None" && expires == "None" {
        println!("\n🚨 {} This page is NOT being cached!", "Warning:".red().bold());
    } else if cache_control.contains("no-cache") || cache_control.contains("no-store") || cache_control.contains("max-age=0") {
        println!("\n🚨 {} This page is NOT being cached!", "Warning:".red().bold());
    } else {
        println!("\n✅ {} This page is being cached!", "Success:".green().bold());
    }
}
