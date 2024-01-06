// src/meal/local.rs
use super::{MEALDriver, MEALArgs};
use std::fmt;
use std::collections::HashMap;
use async_trait::async_trait;


// Std libraries
use std::sync::{Arc, Mutex};
use std::process::Stdio;

// tokio libraries
use tokio::sync::mpsc;
use tokio::process::Command;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;


// Create the LocalDriver struct
pub struct LocalDriver {
    static_fields: HashMap<String, String>,
    model_params: HashMap<String, String>,
    connection_params: HashMap<String, String>,
}

#[async_trait]
impl MEALDriver for LocalDriver {

    ////////////////////////////////////////////////////
    ///// Management of the LocalDriver instance /////
    ////////////////////////////////////////////////////

    fn new(meal_args: MEALArgs) -> Self {
        Self {
            static_fields: meal_args.meal_config[0].clone(),
            connection_params: meal_args.meal_config[1].clone(),
            model_params: meal_args.meal_config[2].clone(),
        }
    }

    //////////////////////////////////////////////////////
    ////// Management of the LocalDriver connection //////
    //////////////////////////////////////////////////////
    async fn spawn_model(&mut self) -> Result<(mpsc::Sender<String>, mpsc::Receiver<String>, mpsc::Receiver<String>), String> {
        // Get the model path
        let model_path = self.model_params.get("modelPath").ok_or_else(|| {
            log::error!("Failed to get the model path");
            "Failed to get the model path".to_string()
        })?;
        // Get the model command
        let model_command = self.model_params.get("inferenceCommand").ok_or_else(|| {
            log::error!("Failed to get the model command");
            "Failed to get the model command".to_string()
        })?;
        // Get the ready token
        let ready_token = self.model_params.get("readyToken").ok_or_else(|| {
            log::error!("Failed to get the ready token");
            "Failed to get the ready token".to_string()
        })?;
        // Get the exit token
        let exit_token = self.model_params.get("exitToken").ok_or_else(|| {
            log::error!("Failed to get the exit token");
            "Failed to get the exit token".to_string()
        })?;
        // Get the start token
        let start_token = self.model_params.get("startToken").ok_or_else(|| {
            log::error!("Failed to get the start token");
            "Failed to get the start token".to_string()
        })?;
        // Get the stop token
        let stop_token = self.model_params.get("stopToken").ok_or_else(|| {
            log::error!("Failed to get the stop token");
            "Failed to get the stop token".to_string()
        })?;

        // Log the model parameters
        log::info!(
            "Model parameters:\n    - Model path: {:#?}\n    - Model command: {:#?}\n    - Ready token: {:#?}\n    - Exit token: {:#?}",
            model_path, model_command, ready_token, exit_token
        );

        // Check if the model path exists
        if !std::path::Path::new(model_path).exists() {
            log::error!("The model path does not exist: {:#?}", model_path);
            return Err("The model path does not exist: ".to_string() + model_path);
        }

        // Combine the cd into model path and model command into one string
        let model_command = format!("cd {} && {}", model_path, model_command);

        // Create Tokio channels for communication
        let (stdin_tx, mut stdin_rx) = mpsc::channel::<String>(1);
        let (stdout_tx, mut stdout_rx) = mpsc::channel::<String>(1);
        let (stderr_tx, stderr_rx) = mpsc::channel::<String>(1);

        let cloned_stdout_rx = Arc::new(Mutex::new(stdout_rx));

        // Spawn a Tokio task to handle command execution
        tokio::spawn(async move {
            // Create a new Command
            let mut child = Command::new("sh")
                .arg("-c")
                .arg(&model_command)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to start command");

            // Get the stdin, stdout, and stderr handles
            let mut stdin = child.stdin.take().expect("Failed to open stdin");
            let mut stdout = child.stdout.take().expect("Failed to open stdout");
            let mut stderr = child.stderr.take().expect("Failed to open stderr");

            // Wait for the ready token
            let mut ready = false;
            while !ready {
                // Check if the ready token is in the stdout channel
                let cloned_stdout_rx = Arc::clone(&cloned_stdout_rx);
                while let Some(chunk) = cloned_stdout_rx.lock().unwrap().recv().await {
                    let stdout_str = String::from_utf8_lossy(&chunk.as_bytes());
                    if stdout_str.contains(ready_token) {
                        ready = true;
                    } else {
                        log::info!("stdout: {:#?}", stdout_str);
                    }
                }
            }

            // Start the main loop
            let mut exit = false;
            while !exit {
                // Read data from the command's standard output and send it to the channel
                let mut buffer = String::new();
                while stdout.read_to_string(&mut buffer).await.expect("Failed to read from stdout") > 0 {
                    // Check if the exit token is in the buffer
                    if buffer.contains(exit_token) {
                        exit = true;
                    }
                    // Check if start  tokes is in the buffer
                    if buffer.contains(start_token) {
                        // Remove the start token from the buffer
                        buffer = buffer.replace(start_token, "");
                    }
                    // Check if stop token is in the buffer
                    if buffer.contains(stop_token) {
                        // Remove the stop token from the buffer
                        buffer = buffer.replace(stop_token, "");
                    }
                    // Send the buffer to the channel
                    stdout_tx.send(buffer.clone()).await.expect("Failed to send stdout data");
                    buffer.clear();
                }

                // Read data from the command's standard error and send it to the channel
                buffer.clear();
                while stderr.read_to_string(&mut buffer).await.expect("Failed to read from stderr") > 0 {
                    stderr_tx.send(buffer.clone()).await.expect("Failed to send stderr data");
                    buffer.clear();
                }

                // Check if there is data in the stdin channel
                if let Ok(stdin_buf) = stdin_rx.try_recv() {
                    // Check if the exit token is in the stdin channel
                    if stdin_buf.contains(exit_token) {
                        exit = true;
                    }
                    // Write the start token to the command's standard input
                    stdin.write_all(start_token.clone().as_bytes()).await.expect("Failed to write to stdin");
                    // Write the data to the command's standard input
                    stdin.write_all(stdin_buf.as_bytes()).await.expect("Failed to write to stdin");
                    // Write the stop token to the command's standard input
                    stdin.write_all(stop_token.clone().as_bytes()).await.expect("Failed to write to stdin");
                }
            }

            // Wait for the child process to finish
            let status = child.wait().await.expect("Failed to wait for child process");
            
            if status.success() {
                log::info!("Model exited successfully");
            } else {
                log::error!("Model exited with status: {}", status);
            }
        });

        // Return the channels
        Ok((stdin_tx, stdout_rx, stderr_rx))
    }


}

// Implementation of debug for LocalDriver
impl fmt::Debug for LocalDriver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Print all the fields of LocalDriver
        f.debug_struct("LocalDriver")
            .field("static_fields", &self.static_fields)
            .field("model_params", &self.model_params)
            .field("connection_params", &self.connection_params)
            .finish()
    }
}