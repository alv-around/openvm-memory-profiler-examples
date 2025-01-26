use axum::{routing::get, Router};
use openvm_prof::recorder::install_prometheus_recorder;
use std::time::Duration;
use tokio::net::TcpListener;
// use tracing::{span, Level};

use openvm_node::proof_and_verify_fib;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let recorder = install_prometheus_recorder();
    recorder.spawn_process_metrics_thread(Duration::from_secs(10));

    let app = Router::new()
        .route(
            "/metrics",
            get(|| {
                let metrics = recorder.handle().render();
                std::future::ready(metrics)
            }),
        )
        .route(
            "/example",
            get(|| {
                // FIXME: calling this endpoint will memory starve /metrics
                proof_and_verify_fib().unwrap();
                std::future::ready("proving job started")
            }),
        );

    let listener = TcpListener::bind("127.0.0.1:9000").await.unwrap();
    println!("Listening on: 127.0.0.1:9000");
    axum::serve(listener, app).await.unwrap();
}
