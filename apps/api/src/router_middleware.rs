use std::time::Instant;

use crate::observability;
use axum::{extract::Request, middleware::Next, response::Response};
use log::info;

pub async fn trace_time(request: Request, next: Next) -> Response {
    let start_time = Instant::now();
    let path = request.uri().path().to_string();
    let method = request.method().to_string();

    observability::HTTP_REQUEST_DURATION
        .with_label_values(&[&method, &path]) // Label with method and path
        .observe(start_time.elapsed().as_secs_f64()); // Observe the duration in seconds

    let response = next.run(request).await;

    // Log the time taken for debugging (optional)
    info!(
        "{}:{} time taken: {}ms",
        path,
        method,
        start_time.elapsed().as_millis()
    );

    response
}

pub async fn total_http_request(request: Request, next: Next) -> Response {
    observability::HTTP_REQUESTS_TOTAL.inc();
    let response = next.run(request).await;
    response
}

//TODO: probably better for the active conncetion on updating areas?
pub async fn active_connections(request: Request, next: Next) -> Response {
    observability::ACTIVE_CONNECTIONS.inc();
    let response = next.run(request).await;
    observability::ACTIVE_CONNECTIONS.dec();
    response
}
