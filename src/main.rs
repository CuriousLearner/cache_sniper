mod cache_checker;
mod http_client;
mod utils;

use clap::Parser;
use cache_checker::check_cache;

/// CLI arguments
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Target URL to check
    #[arg(short, long)]
    url: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match check_cache(&args.url).await {
        Ok(_) => println!("\nCache analysis complete."),
        Err(e) => eprintln!("Error: {}", e),
    }
}
