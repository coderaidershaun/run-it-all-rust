use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::task;
use reqwest::Error;

async fn query_echo_endpoint() -> Result<(), Error> {
    let echo_url = "http://localhost:8080";
    let client = reqwest::Client::new();

    
    // Query the echo endpoint periodically
    loop {

        // Wait for 5 seconds before querying again
        // NOTE HOW WE HAVE TO WAIT FIRST SO WEB SERVER HAS A CHANCE TO START
        tokio::time::sleep(Duration::from_secs(5)).await;

        let response = client.get(echo_url).send().await?;
        dbg!("HELLO");
        let text = response.text().await?;
        println!("Response from the echo endpoint: {}", text);
        // Wait for 5 seconds before querying again
        tokio::time::sleep(Duration::from_secs(5)).await;

    }
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

    // Spawn a task to query the echo endpoint
    let query_task = task::spawn(query_echo_endpoint());

    // Wait for the web server process to exit
    let server_result = web_server.wait();

    // Cancel the echo endpoint query task when the server exits
    query_task.abort();

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
