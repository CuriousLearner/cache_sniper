use warp::Filter;
use redis::Commands;
use std::net::SocketAddr;

/// Connects to Redis
fn get_redis_connection() -> Result<redis::Connection, redis::RedisError> {
    let client = redis::Client::open("redis://127.0.0.1:6379")?;
    client.get_connection()
}

/// Gets the current metrics from Redis
fn get_metrics() -> (usize, usize) {
    match get_redis_connection() {
        Ok(mut conn) => {
            let requests: usize = conn.get("cache_sniper_requests").unwrap_or(0);
            let errors: usize = conn.get("cache_sniper_errors").unwrap_or(0);
            (requests, errors)
        }
        Err(_) => (0, 0)
    }
}

/// Increments the request count in Redis (no-op if Redis is unavailable)
#[allow(dead_code)]
pub fn increment_requests() {
    if let Ok(mut conn) = get_redis_connection() {
        let _: Result<(), _> = conn.incr("cache_sniper_requests", 1);
    }
}

/// Increments the error count in Redis (no-op if Redis is unavailable)
#[allow(dead_code)]
pub fn increment_errors() {
    if let Ok(mut conn) = get_redis_connection() {
        let _: Result<(), _> = conn.incr("cache_sniper_errors", 1);
    }
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
