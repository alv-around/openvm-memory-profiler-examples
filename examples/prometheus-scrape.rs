use axum::{routing::get, Router};
use metrics_process::Collector;
use openvm_prof::recorder::install_prometheus_recorder;
use tokio::{net::TcpListener, task};

use openvm_node::proof_and_verify_fib;

#[tokio::main]
async fn main() {
    let recorder = install_prometheus_recorder();
    let collector = Collector::default();
    collector.describe();

    let handle = recorder.handle().clone();
    let app = Router::new()
        .route(
            "/metrics",
            get(move || {
                collector.collect();
                let metrics = handle.render();
                handle.run_upkeep();
                std::future::ready(metrics)
            }),
        )
        .route(
            "/example",
            get(|| {
                task::spawn_blocking(|| {
                    proof_and_verify_fib().unwrap();
                });
                std::future::ready("proving job started")
            }),
        );

    let listener = TcpListener::bind("127.0.0.1:9000").await.unwrap();
    println!("Listening on: 127.0.0.1:9000");
    axum::serve(listener, app).await.unwrap();
}
