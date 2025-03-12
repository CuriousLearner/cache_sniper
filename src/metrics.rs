use prometheus::{Encoder, TextEncoder, Registry, Counter};
use warp::{Filter};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref REQUEST_COUNTER: Counter = Counter::new("requests_total", "Total requests").unwrap();
}

pub async fn start_metrics_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let registry = Registry::new();
    registry.register(Box::new(REQUEST_COUNTER.clone())).unwrap();

    let metrics_route = warp::path!("metrics").map(move || {
        let mut buffer = Vec::new();
        let encoder = TextEncoder::new();
        let metric_families = registry.gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        warp::reply::with_header(buffer, "Content-Type", "text/plain; version=0.0.4")
    });

    warp::serve(metrics_route).run(([0, 0, 0, 0], 9090)).await;
    Ok(())
}
