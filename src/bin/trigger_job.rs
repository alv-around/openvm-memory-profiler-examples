// Spawn a new task that makes a request to the `/example` endpoint
#[tokio::main]
async fn main() {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    // Make the HTTP request using the `reqwest` crate
    let client = reqwest::Client::new();
    match client.get("http://127.0.0.1:9000/example").send().await {
        Ok(response) => {
            println!("Response from /example: {:?}", response.status());
        }
        Err(e) => {
            eprintln!("Failed to call /example: {:?}", e);
        }
    };
}
// Wait a short time to ensure the server is ready
