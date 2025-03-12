mod cache_checker;
mod http_client;
mod utils;
mod metrics;

use clap::Parser;
use cache_checker::check_cache;
use metrics::start_metrics_server;
use tokio::sync::mpsc;
use std::error::Error;

/// CLI arguments
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Target URLs to check
    #[arg(short, long, num_args = 1..)]
    urls: Vec<String>,

    /// Enable JSON output
    #[arg(short, long)]
    json: bool,

    /// Enable Prometheus metrics server
    #[arg(long)]
    metrics: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Start Prometheus metrics server if enabled
    if args.metrics {
        tokio::spawn(async {
            start_metrics_server().await.unwrap();
        });
    }

    let (tx, mut rx) = mpsc::channel(args.urls.len());

    // Spawn multiple tasks for parallel scanning
    for url in &args.urls {
        let url = url.clone();
        let tx = tx.clone();
        tokio::spawn(async move {
            let result: Result<_, Box<dyn Error + Send + Sync>> = check_cache(&url).await.map_err(|e| e.into());
            tx.send((url, result)).await.unwrap();
        });
    }

    drop(tx);

    let mut results = vec![];
    while let Some((url, result)) = rx.recv().await {
        match result {
            Ok(json_result) => {
                results.push(json_result);
            },
            Err(e) => eprintln!("Error scanning {}: {}", url, e),
        }
    }

    if args.json {
        println!("{}", serde_json::to_string_pretty(&results).unwrap());
    }
}
