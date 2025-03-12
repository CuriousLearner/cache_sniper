use comfy_table::{Table, Row, ContentArrangement};
use colored::*;

/// Prints cache header results in a nicely formatted table with emojis.
pub fn print_results(url: &str, cache_control: &str, etag: &str, last_modified: &str, expires: &str) {
    println!("\n🌍 Scanning: {}\n", url.bright_cyan());

    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        "🔍 Header".bold().to_string(),
        "📜 Value".bold().to_string(),
    ]);

    table.add_row(Row::from(vec!["🛠 Cache-Control".to_string(), cache_control.to_string()]));
    table.add_row(Row::from(vec!["🔄 ETag".to_string(), etag.to_string()]));
    table.add_row(Row::from(vec!["📆 Last-Modified".to_string(), last_modified.to_string()]));
    table.add_row(Row::from(vec!["⏳ Expires".to_string(), expires.to_string()]));

    println!("{}", table);

    if cache_control.contains("no-cache") || cache_control.contains("no-store") {
        println!("\n🚨 {} This page is NOT being cached!", "Warning:".red().bold());
    } else if !etag.is_empty() || !last_modified.is_empty() {
        println!("\n✅ {} This page is being cached!", "Success:".green().bold());
    } else {
        println!("\n⚠️ {} Cache status uncertain.", "Caution:".yellow().bold());
    }
}
