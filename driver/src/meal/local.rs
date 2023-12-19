// src/meal/local.rs
use super::{MEALDriver, MEALArgs};
use async_trait::async_trait;
use std::collections::HashMap;
use std::process::Stdio;
use tokio::process::Command;


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
 
    async fn spawn_model(&self) -> Result<tokio::process::Child, String> {
        // Get the model path and check wether it exists on the filesystem
        let model_path = match self.model_params.get("path") {
            Some(path) => path,
            None => {
                log::error!("Failed to get the model path");
                return Err("Failed to get the model path".to_string());
            }
        };

        // Check if the model path exists
        if !std::path::Path::new(model_path).exists() {
            log::error!("The model path does not exist: {:#?}", model_path);
            return Err("The model path does not exist: ".to_string() + model_path);
        }

        // Combine the cd into model path and model command into one string
        let model_command = match self.model_params.get("command") {
            Some(command) => command,
            None => {
                log::error!("Failed to get the model command");
                return Err("Failed to get the model command".to_string());
            }
        };

        // Combine the cd into model path and model command into one string
        let model_command = "cd ".to_string() + model_path + " && " + model_command;

        // Spawn a new model and check if it was successful
        let mut model = match Command::new("sh")
            .arg("-c")
            .arg(model_command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(model) => model,
            Err(error) => {
                log::error!("Failed to spawn the model: {:#?}", error);
                return Err("Failed to spawn the model: ".to_string() + &error.to_string());
            }
        };

        // Return the model pipes
        Ok(model)
    }

}
