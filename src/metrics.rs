use warp::Filter;
use redis::Commands;
use std::net::SocketAddr;

/// Connects to Redis
fn get_redis_connection() -> redis::Connection {
    let client = redis::Client::open("redis://127.0.0.1:6379").expect("Failed to connect to Redis");
    client.get_connection().expect("Failed to connect to Redis server")
}

/// Gets the current metrics from Redis
fn get_metrics() -> (usize, usize) {
    let mut conn = get_redis_connection();
    let requests: usize = conn.get("cache_sniper_requests").unwrap_or(0);
    let errors: usize = conn.get("cache_sniper_errors").unwrap_or(0);
    (requests, errors)
}

/// Increments the request count in Redis
pub fn increment_requests() {
    let mut conn = get_redis_connection();
    let _: () = conn.incr("cache_sniper_requests", 1).unwrap();
}

/// Increments the error count in Redis
pub fn increment_errors() {
    let mut conn = get_redis_connection();
    let _: () = conn.incr("cache_sniper_errors", 1).unwrap();
}

/// Starts the Prometheus metrics HTTP server
pub async fn start_metrics_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let metrics_route = warp::path("metrics")
        .and(warp::get())
        .and_then(handle_metrics);

    let home_route = warp::path::end()
        .map(|| warp::reply::html("<h1>🚀 CacheSniper Metrics</h1><p>Visit <a href='/metrics'>/metrics</a> to see live stats.</p>"));

    let routes = metrics_route.or(home_route);

    let addr: SocketAddr = "127.0.0.1:9090".parse()?;
    println!("📡 Serving metrics on {}", addr);

    warp::serve(routes).run(addr).await;

    Ok(())
}

/// Handles the `/metrics` endpoint and fetches live data from Redis
async fn handle_metrics() -> Result<impl warp::Reply, warp::Rejection> {
    let (requests, errors) = get_metrics();
    let response = format!(
        "cache_sniper_requests_total {}\ncache_sniper_errors_total {}\n",
        requests, errors
    );

    Ok(warp::reply::with_header(response, "Content-Type", "text/plain; version=0.0.4"))
}
