use std::time::Instant;

use axum::{extract::Request, middleware::Next, response::Response};
use log::info;

pub async fn trace_time(request: Request, next: Next) -> Response {
    let start_time = Instant::now();
    let path = request.uri().path().to_string();
    let response = next.run(request).await;
    info!(
        "{} time taken: {}ms",
        path,
        start_time.elapsed().as_millis()
    );
    response
}
