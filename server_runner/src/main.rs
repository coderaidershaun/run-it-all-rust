use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::task;
use webbrowser;

use std::fs::{self, File};


async fn query_backend_server() -> Result<(), reqwest::Error> {
  let backend_url: &str = "http://localhost:8080";
  let client: reqwest::Client = reqwest::Client::new();

  // Wait for 5 seconds before querying again
  tokio::time::sleep(Duration::from_secs(5)).await;
  
  // Query the echo endpoint periodically
  let response: reqwest::Response = client.get(backend_url).send().await?;
  let text: String = response.text().await?;
  println!("Response from backend endpoint: {}", text);
  Ok(())   
}


async fn query_frontend_server() -> Result<(), reqwest::Error> {
  let frontend_url: &str = "http://localhost:5173";
  let client: reqwest::Client = reqwest::Client::new();

  // Wait for 5 seconds before querying again
  tokio::time::sleep(Duration::from_secs(5)).await;
  
  // Query the echo endpoint periodically
  let response: reqwest::Response = client.get(frontend_url).send().await?;
  let text: String = response.text().await?;
  println!("Response from frontend endpoint: HELLO FRONTEND!");

  if webbrowser::open(frontend_url).is_ok() {
    println!("Successfully opened the browser.");
  } else {
      println!("Failed to open the browser.");
  }

  Ok(())   
}


#[tokio::main]
async fn main() {
  println!("Starting the web server...");

    // Clear paths
    fs::remove_dir_all("../frontend").unwrap();
    fs::remove_dir_all("../selfbuild").unwrap();

    // Create a folder
    let folder_path: &str = "../selfbuild";
    fs::create_dir_all(folder_path).unwrap();

    // List of folder names you want to create
    let folder_names: [&str; 2] = ["src", "target"];

    for folder_name in &folder_names {
      let folder_path = format!("{}/{}", folder_path, folder_name);
      fs::create_dir_all(&folder_path).unwrap();
    }

    // Pre-existing content for Cargo.toml
    let cargo_toml_content: &str = r#"
[package]
name = "my_project"
version = "0.1.0"
edition = "2018"

[dependencies]
actix-web = "4.0"
"#;

    // Create Cargo.toml file inside the folder with the pre-existing content
    let cargo_toml_path: &str = "../selfbuild/Cargo.toml";
    fs::write(cargo_toml_path, cargo_toml_content).unwrap();

    // Create main script
    let main_content = r#"fn main () {
  println!("Hello my main function");
}
"#;

    // Create Cargo.toml file inside the folder with the pre-existing content
    let cargo_main_path: &str = "../selfbuild/src/main.rs";
    fs::write(cargo_main_path, main_content).unwrap();


  // Build selfbuild app
  let mut selfbuild: std::process::ExitStatus = Command::new("cargo")
    .arg("build")
    .current_dir("../selfbuild") // Adjust the path to your web server project directory
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .status()
    .expect("Failed to build the selfbuild application");



  // Run selfbuild app
  let selfbuild_run_status: std::process::ExitStatus = Command::new("cargo")
    .arg("run")
    .current_dir("../selfbuild") // Adjust the path to your web server project directory
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .status()
    .expect("Failed to run the selfbuild application");


  // Create a folder
  let folder_path: &str = "../frontend";
  fs::create_dir_all(folder_path).unwrap();


  // Create frontend app
  let frontend: std::process::ExitStatus = Command::new("yarn")
    .arg("create")
    .arg("vite")
    .arg(".")
    .arg("--template")
    .arg("react-ts")
    .current_dir("../frontend") // Adjust the path to your web server project directory
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .status()
    .expect("Failed to create react application");


  // Build frontend app
  let frontend: std::process::ExitStatus = Command::new("yarn")
    .arg("--exact")
    .current_dir("../frontend") // Adjust the path to your web server project directory
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .status()
    .expect("Failed to build react application");


  // Run backend server
  let mut backend_server: std::process::Child = Command::new("cargo")
    .arg("run")
    .current_dir("../backend_server") // Adjust the path to your web server project directory
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .spawn()
    .expect("Failed to start the backend web server");

  // Run frontend server
  let mut frontend_server: std::process::Child = Command::new("yarn")
    .arg("dev")
    .current_dir("../frontend") // Adjust the path to your web server project directory
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .spawn()
    .expect("Failed to start the frontend web server");

  // Spawn a task to query the echo endpoint
  let query_backend_task = task::spawn(query_backend_server());

  // Spawn a task to query the echo endpoint
  let query_frontend_task = task::spawn(query_frontend_server());


  // Wait for the web server process to exit
  let backend_server_result = backend_server.wait();

  // Wait for the web server process to exit
  let frontend_server_result = backend_server.wait();

  // Cancel the echo endpoint query task when the server exits
  query_backend_task.abort();
  query_frontend_task.abort();

  match backend_server_result {
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
