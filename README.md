# OpenVM Memory Profiler

In this repo there are two examples how to use the memory profiler for Openvm:

1. `prometheus-scrape.rs` -> the "standard" process in prometheus,
allow prometheus to get the metrics through "/metrics" endpoint.
1. `prometheus-push.rs` -> a script to sends metrics through the prometheus push-gateway.

## Usage

1. Start the grafana and prometheus services with the following command:

  ```console
  docker compose up -d
  ```

1. run either `prometheus-scrape` or `prometheus-push` with:

  ```console
  cargo run --example <EXAMPLE_NAME>
  ```
