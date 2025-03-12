use comfy_table::{Table, Row, Cell, ContentArrangement, presets::UTF8_FULL, Attribute};
use colored::*;

/// Prints cache header results in a properly formatted table with emojis.
pub fn print_results(url: &str, cache_control: &str, etag: &str, last_modified: &str, expires: &str) {
    println!("\n🌍 Scanning: {}\n", url.bright_cyan());

    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.load_preset(UTF8_FULL);

    table.set_header(vec![
        Cell::new("🔍 Header").add_attribute(Attribute::Bold),
        Cell::new("📜 Value").add_attribute(Attribute::Bold),
    ]);

    table.add_row(Row::from(vec![Cell::new("🛠 Cache-Control"), Cell::new(cache_control)]));
    table.add_row(Row::from(vec![Cell::new("🔄 ETag"), Cell::new(etag)]));
    table.add_row(Row::from(vec![Cell::new("📆 Last-Modified"), Cell::new(last_modified)]));
    table.add_row(Row::from(vec![Cell::new("⏳ Expires"), Cell::new(expires)]));

    println!("{}", table);

    if cache_control == "None" && etag == "None" && last_modified == "None" && expires == "None" {
        println!("\n🚨 {} This page is NOT being cached!", "Warning:".red().bold());
    } else if cache_control.contains("no-cache") || cache_control.contains("no-store") || cache_control.contains("max-age=0") {
        println!("\n🚨 {} This page is NOT being cached!", "Warning:".red().bold());
    } else {
        println!("\n✅ {} This page is being cached!", "Success:".green().bold());
    }
}
