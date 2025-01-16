use async_nats::jetstream::Context as JetStreamContext;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

// Rate-limiting settings
const REQUEST_LIMIT: usize = 2; // Maximum allowed requests
const TIME_WINDOW: Duration = Duration::from_secs(5); // Time window in seconds

#[derive(Serialize, Deserialize)]
struct RateLimitEntry {
    request_count: usize,
    last_reset: u64, // Unix timestamp in seconds
}

pub async fn rate_limiter(
    req: Request<Body>,
    next: Next,
    jetstream: Arc<JetStreamContext>,
) -> impl IntoResponse {
    let ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|header| header.to_str().ok())
        .map(String::from)
        .unwrap_or_else(|| {
            req.extensions()
                .get::<std::net::SocketAddr>()
                .map(|addr| addr.ip().to_string())
                .unwrap_or_default()
        });

    // Key for the rate limiter
    let key = format!("rate-limit.{}{}", ip, "localhost");

    println!("Rate limiter key: {}", key);

    let kv = jetstream
        .create_key_value(async_nats::jetstream::kv::Config {
            bucket: "rate_limits".to_string(),
            history: 10,
            ..Default::default()
        })
        .await
        .unwrap();

    // Get the rate-limiting state
    let mut entry: RateLimitEntry = match kv.get(&key).await {
        Ok(Some(kv_entry)) => {
            serde_json::from_slice(&kv_entry).unwrap_or_else(|_| RateLimitEntry {
                request_count: 0,
                last_reset: 0,
            })
        }
        _ => RateLimitEntry {
            request_count: 0,
            last_reset: 0,
        },
    };

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    if now - entry.last_reset > TIME_WINDOW.as_secs() {
        // Reset if time window expired
        entry.request_count = 0;
        entry.last_reset = now;
    }

    if entry.request_count >= REQUEST_LIMIT {
        // Exceeded request limit
        return (StatusCode::TOO_MANY_REQUESTS, "Too many requests").into_response();
    }

    // Increment count and update KV store
    entry.request_count += 1;
    let value = serde_json::to_vec(&entry).unwrap();
    kv.put(&key, value.into()).await.unwrap();

    // Proceed to the next middleware or handler
    next.run(req).await
}
