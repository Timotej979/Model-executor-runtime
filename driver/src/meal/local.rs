// src/meal/local.rs
use super::{MEALDriver, MEALArgs};
use async_trait::async_trait;
use std::collections::HashMap;

// Std libraries
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
            model_params: meal_args.meal_config[1].clone(),
            connection_params: meal_args.meal_config[2].clone(),
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

        // Log the model parameters
        log::info!(
            "Model parameters:\n    - Model path: {:#?}\n    - Model command: {:#?}",
            model_path, model_command
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
        let (stdout_tx, stdout_rx) = mpsc::channel::<String>(1);
        let (stderr_tx, stderr_rx) = mpsc::channel::<String>(1);

        // Spawn a Tokio task to handle command execution
        tokio::spawn(async move {
            // Log the model command
            log::info!("Model command: {:#?}", model_command);

            // Create a new Command
            let mut child = Command::new(model_command)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to start command");

            // Get the stdin, stdout, and stderr handles
            let mut stdin = child.stdin.take().expect("Failed to open stdin");
            let mut stdout = child.stdout.take().expect("Failed to open stdout");
            let mut stderr = child.stderr.take().expect("Failed to open stderr");

            // Send input data to the command's standard input
            if let Some(input_data) = stdin_rx.recv().await {
                stdin.write_all(input_data.as_bytes()).await.expect("Failed to write to stdin");
            }

            // Read data from the command's standard output and send it to the channel
            let mut buffer = String::new();
            while stdout.read_to_string(&mut buffer).await.expect("Failed to read from stdout") > 0 {
                stdout_tx.send(buffer.clone()).await.expect("Failed to send stdout data");
                buffer.clear();
            }

            // Read data from the command's standard error and send it to the channel
            buffer.clear();
            while stderr.read_to_string(&mut buffer).await.expect("Failed to read from stderr") > 0 {
                stderr_tx.send(buffer.clone()).await.expect("Failed to send stderr data");
                buffer.clear();
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