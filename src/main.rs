mod cache_checker;
mod validate_cache;
mod http_client;
mod utils;
mod metrics;

use clap::Parser;
use cache_checker::check_cache;
use validate_cache::validate_cache;
use tokio::sync::mpsc;
use std::error::Error;
use std::fs;

/// CLI arguments
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Target URLs to check (accepts single or multiple URLs)
    #[arg(short = 'u', long = "url", alias = "urls", num_args = 1..)]
    urls: Vec<String>,

    /// Enable JSON output
    #[arg(short, long)]
    json: bool,

    /// Enable cache validation (checks if revalidation works correctly)
    #[arg(long)]
    validate: bool,

    /// Start Prometheus metrics server
    #[arg(long)]
    metrics: bool,

    /// Save output to a JSON file
    #[arg(short, long)]
    output: Option<String>,

    /// Exit with code 1 if any URL has no caching enabled
    #[arg(long)]
    exit_code: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Handle metrics separately
    if args.metrics {
        println!("📡 Starting Prometheus Metrics Server at http://localhost:9090/metrics");
        metrics::start_metrics_server().await.unwrap();
        return;
    }

    let (tx, mut rx) = mpsc::channel(args.urls.len().max(1)); // Ensure buffer size is at least 1

    for url in &args.urls {
        let url = url.clone();
        let tx = tx.clone();
        let validate = args.validate;
        tokio::spawn(async move {
            let result: Result<serde_json::Value, Box<dyn Error + Send + Sync>> = if validate {
                validate_cache(&url).await.map(|r| serde_json::to_value(r).unwrap()).map_err(|e| e.into())
            } else {
                check_cache(&url, false).await.map(|r| serde_json::to_value(r).unwrap()).map_err(|e| e.into())
            };

            tx.send((url, result)).await.unwrap();
        });
    }

    drop(tx);

    let mut results = vec![];
    let mut has_errors = false;
    let mut has_no_cache = false;

    while let Some((url, result)) = rx.recv().await {
        match result {
            Ok(json_result) => {
                // Check if this result indicates no caching
                if let Some(obj) = json_result.as_object() {
                    if let Some(is_cached) = obj.get("is_cached") {
                        if is_cached == &serde_json::Value::Bool(false) {
                            has_no_cache = true;
                        }
                    }
                    if let Some(is_consistent) = obj.get("is_consistent") {
                        if is_consistent == &serde_json::Value::Bool(false) {
                            has_no_cache = true;
                        }
                    }
                }
                results.push(json_result);
            },
            Err(e) => {
                eprintln!("Error scanning {}: {}", url, e);
                has_errors = true;
            }
        }
    }

    if let Some(output_file) = &args.output {
        let json_output = serde_json::to_string_pretty(&results).unwrap();
        fs::write(output_file, json_output).expect("Failed to write output file");
        println!("
📁 Results saved to {}", output_file);
    }

    if args.json {
        println!("{}", serde_json::to_string_pretty(&results).unwrap());
    }

    // Exit with error code if requested and caching issues detected
    if args.exit_code && (has_errors || has_no_cache) {
        std::process::exit(1);
    }
}
