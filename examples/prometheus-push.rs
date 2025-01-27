use metrics_exporter_prometheus::PrometheusBuilder;
use openvm_prof::recorder::install_prometheus_recorder;
use std::time::Duration;

use openvm_node::proof_and_verify_fib;

#[tokio::main]
async fn main() {
    let (_, _) = PrometheusBuilder::new()
        .with_push_gateway(
            "http://127.0.0.1:9091/metrics/job/openvm",
            Duration::from_secs(10),
            None,
            None,
        )
        .unwrap()
        .build()
        .unwrap();

    println!("connected to prometheus push-gateway");

    let recorder = install_prometheus_recorder();
    recorder.spawn_process_metrics_thread(Duration::from_secs(10));
    println!("recorder built");

    proof_and_verify_fib().unwrap();
}
