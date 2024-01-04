// src/meal/local.rs
use super::{MEALDriver, MEALArgs};
use async_trait::async_trait;
use std::collections::HashMap;

// Std libraries
use std::sync::Arc;
use std::process::Stdio;

// tokio libraries
use tokio::sync::{mpsc, Mutex};
use tokio::process::Command;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;


// Create the ModelChannels struct
struct ModelChannels {
    stdin_tx: mpsc::Receiver<Vec<String>>,
    stdout_rx: mpsc::Sender<Vec<String>>,
    stderr_rx: mpsc::Sender<Vec<String>>,
}

// Create the LocalDriver struct
pub struct LocalDriver {
    static_fields: HashMap<String, String>,
    model_params: HashMap<String, String>,
    connection_params: HashMap<String, String>,
    model_channels: Option<ModelChannels>,
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
            model_channels: None,
        }
    }

    //////////////////////////////////////////////////////
    ////// Management of the LocalDriver connection //////
    //////////////////////////////////////////////////////
    async fn spawn_model(&mut self) -> Result<(), String> {
        // Get the model path
        let model_path = self.model_params.get("path").ok_or_else(|| {
            log::error!("Failed to get the model path");
            "Failed to get the model path".to_string()
        })?;

        // Get the model command
        let model_command = self.model_params.get("command").ok_or_else(|| {
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

        // Spawn a new model and check if it was successful
        let mut child = match Command::new("sh")
            .arg("-c")
            .arg(model_command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(child) => child,
            Err(error) => {
                log::error!("Failed to spawn the model: {:#?}", error);
                return Err("Failed to spawn the model: ".to_string() + &error.to_string());
            }
        };

        // Create channels for stdin, stdout, and stderr
        let (stdin_tx, mut stdin_rx) = mpsc::channel::<Vec<String>>(32);
        let (stdout_tx, stdout_rx) = mpsc::channel::<Vec<String>>(32);
        let (stderr_tx, stderr_rx) = mpsc::channel::<Vec<String>>(32);

        let mut temp_rx = mpsc::channel::<Vec<String>>(32).1;
        std::mem::swap(&mut stdin_rx, &mut temp_rx);

        // Initialize the model_channels field
        self.model_channels = Some(ModelChannels {
            stdin_tx: temp_rx,
            stdout_rx: stdout_tx.clone(),
            stderr_rx: stderr_tx.clone(),
        });

        // Spawn tasks to handle reading from pipes and writing to channels
        tokio::spawn(async move {
            while let Some(data) = stdin_rx.recv().await {
                // Write data to the child process stdin
                if let Err(err) = child.stdin.as_mut().unwrap().write_all(data.join("\n").as_bytes()).await {
                    log::error!("Failed to write to child process stdin: {:?}", err);
                    break;
                }
            }
        });
        tokio::spawn(async move {
            let mut buf = Vec::new();
            loop {
                match child.stdout.as_mut().unwrap().read_to_end(&mut buf).await {
                    Ok(n) if n > 0 => {
                        if let Err(_) = stdout_tx.send(buf.iter().map(|&b| b.to_string()).collect()).await {
                            break;
                        }
                        buf.clear();
                    }
                    _ => break,
                }
            }
        });

        // Spawn a task to handle reading from stderr and writing to a channel
        tokio::spawn(async move {
            let mut buf = Vec::new();
            loop {
                match child.stderr.as_mut().unwrap().read_to_end(&mut buf).await {
                    Ok(n) if n > 0 => {
                        if let Err(_) = stderr_tx.send(buf.iter().map(|&b| b.to_string()).collect()).await {
                            break;
                        }
                        buf.clear();
                    }
                    _ => break,
                }
            }
        });

        Ok(())  
    }
}