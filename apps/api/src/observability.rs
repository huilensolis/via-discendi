use axum::{body::Body, http::Response};
use lazy_static::lazy_static;
use prometheus::{Counter, Gauge, HistogramOpts, HistogramVec, IntGauge, Registry};
use std::time::Duration;
use sysinfo::{DiskExt, Pid, ProcessExt, System, SystemExt};
use tokio::time;

// Define metrics
lazy_static! {
    static ref REGISTRY: Registry = Registry::new();

    pub static ref HTTP_REQUESTS_TOTAL: Counter = Counter::new(
        "http_requests_total",
        "Total number of HTTP requests"
    ).expect("metric can be created");

    pub static ref HTTP_REQUEST_DURATION: HistogramVec = HistogramVec::new(
        HistogramOpts::new("http_request_duration_seconds", "Time taken for HTTP requests"),
        &["method", "path"], // Define the labels for method and path
    ).expect("metric can be created");

    pub static ref ACTIVE_CONNECTIONS: IntGauge = IntGauge::new(
        "active_connections",
        "Number of currently active connections"
    ).expect("metric can be created");

    // System metrics
    static ref CPU_USAGE_PERCENT: Gauge = Gauge::new(
        "system_cpu_usage_percent",
        "Current CPU usage percentage"
    ).expect("metric can be created");
    static ref MEMORY_USAGE_BYTES: Gauge = Gauge::new(
        "system_memory_usage_bytes",
        "Current memory usage in bytes"
    ).expect("metric can be created");
    static ref MEMORY_TOTAL_BYTES: Gauge = Gauge::new(
        "system_memory_total_bytes",
        "Total system memory in bytes"
    ).expect("metric can be created");
    static ref SWAP_USAGE_BYTES: Gauge = Gauge::new(
        "system_swap_usage_bytes",
        "Current swap usage in bytes"
    ).expect("metric can be created");
    static ref DISK_USAGE_BYTES: Gauge = Gauge::with_opts(
        prometheus::Opts::new(
            "system_disk_usage_bytes",
            "Disk usage in bytes"
        ).const_label("mountpoint", "/")
    ).expect("metric can be created");
}

pub fn register_metrics() {
    // Register HTTP metrics
    let _ = REGISTRY.register(Box::new(HTTP_REQUESTS_TOTAL.clone()));
    let _ = REGISTRY.register(Box::new(HTTP_REQUEST_DURATION.clone()));
    let _ = REGISTRY.register(Box::new(ACTIVE_CONNECTIONS.clone()));

    let _ = REGISTRY.register(Box::new(CPU_USAGE_PERCENT.clone()));
    let _ = REGISTRY.register(Box::new(MEMORY_USAGE_BYTES.clone()));
    let _ = REGISTRY.register(Box::new(MEMORY_TOTAL_BYTES.clone()));
    let _ = REGISTRY.register(Box::new(SWAP_USAGE_BYTES.clone()));
    let _ = REGISTRY.register(Box::new(DISK_USAGE_BYTES.clone()));
}

pub async fn update_system_metrics(mut sys: System) {
    let mut interval = time::interval(Duration::from_secs(1));

    loop {
        interval.tick().await;

        // Update all system information
        sys.refresh_all();

        let current_pid = std::process::id();

        let current_process = sys
            .processes()
            .get(&Pid::from(usize::try_from(current_pid).unwrap()))
            .unwrap();

        // Update CPU usage
        let cpu_usage = current_process.cpu_usage();
        CPU_USAGE_PERCENT.set(cpu_usage as f64);

        // Update memory metrics
        let used_memory = current_process.memory() / (1024 * 1024);
        let total_memory = sys.total_memory() as f64;

        MEMORY_USAGE_BYTES.set(used_memory as f64);
        MEMORY_TOTAL_BYTES.set(total_memory * 1024.0);

        // Update swap metrics
        let used_swap = sys.used_swap() as f64;
        SWAP_USAGE_BYTES.set(used_swap * 1024.0);
        HTTP_REQUEST_DURATION.observe_closure_duration(…)

        // Update disk metrics
        if let Some(disk) = sys
            .disks()
            .iter()
            .find(|disk| disk.mount_point() == std::path::Path::new("/"))
        {
            DISK_USAGE_BYTES.set((disk.total_space() - disk.available_space()) as f64);
        }
    }
}

pub async fn system_metrics_router() -> Response<Body> {
    use prometheus::Encoder;
    let encoder = prometheus::TextEncoder::new();
    let mut buffer = Vec::new();
    encoder.encode(&REGISTRY.gather(), &mut buffer).unwrap();

    return Response::builder()
        .header("Content-Type", "text/plain")
        .body(Body::from(buffer))
        .unwrap();
}
