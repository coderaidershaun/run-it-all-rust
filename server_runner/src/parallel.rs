use std::process::{Command, Stdio};
use std::sync::Arc;
use std::time::Duration;
use tokio::task;

async fn query_echo_endpoint(stop_flag: Arc<()>) -> Result<(), reqwest::Error> {
    let echo_url = "http://localhost:8080";
    let client = reqwest::Client::new();

    // Query the echo endpoint periodically
    loop {
        let response = client.get(echo_url).send().await?;
        let text = response.text().await?;
        println!("Response from the echo endpoint: {}", text);

        // Check if we should stop querying
        if Arc::strong_count(&stop_flag) == 1 {
            break;
        }

        // Wait for 5 seconds before querying again
        tokio::time::sleep(Duration::from_secs(5)).await;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    println!("Starting the web server...");

    let mut web_server = Command::new("cargo")
        .arg("run")
        .current_dir("../backend_server") // Adjust the path to your web server project directory
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start the web server");

    let stop_flag = Arc::new(());
    let stop_flag_clone = Arc::clone(&stop_flag);

    // Spawn a task to query the echo endpoint
    let query_task = task::spawn(async move {
        if let Err(error) = query_echo_endpoint(stop_flag_clone).await {
            eprintln!("Error querying the echo endpoint: {}", error);
        }
    });

    // Wait for the web server process to exit
    let server_result = web_server.wait();

    // Drop the stop_flag Arc to signal the echo endpoint query task to stop
    drop(stop_flag);

    // Wait for the query task to complete
    if let Err(error) = query_task.await {
        eprintln!("Error in the query task: {:?}", error);
    }

    match server_result {
        Ok(status) => {
            if status.success() {
                println!("Web server exited successfully");
            } else {
                eprintln!("Web server exited with an error: {}", status);
            }
        }
        Err(error) => {
            eprintln!("Failed to wait for the web server process: {}", error);
        }
    }
}
