// src/meal/mod.rs
use std::fmt;
use std::result::Result;
use std::collections::HashMap;
use async_trait::async_trait;
use tokio::sync::mpsc;

// Define MEALArgs struct
pub struct MEALArgs {
    pub meal_config: Vec<HashMap<String, String>>,
}

#[async_trait]
pub trait MEALDriver: fmt::Debug + Send + Sync {
    
    // Create the MEALDriver constructor
    fn new(meal_args: MEALArgs) -> Self where Self: Sized;
    
    // MEALDriver methods
    async fn spawn_model(&mut self) -> Result<(mpsc::Sender<String>, mpsc::Receiver<String>, mpsc::Receiver<String>), String>;

}

pub mod local;
pub mod ssh;

// MEAL struct
#[derive(Debug)]
pub struct MEAL {
    driver: Box<dyn MEALDriver>,
}
impl MEAL {
    pub fn create(driver_type: &str, meal_args: MEALArgs) -> Result<Self, String> {
        let driver: Box<dyn MEALDriver> = match driver_type {
            "local" => Box::new(local::LocalDriver::new(meal_args)),
            "ssh" => Box::new(ssh::SSHDriver::new(meal_args)),
            _ => {
                log::error!("Unknown MEAL driver type: {:#?}", driver_type);
                return Err("Unknown MEAL driver type: ".to_string() + driver_type);
            }
        };

        Ok(Self { driver })
    }

    // Get the driver type
    pub fn driver_type(&self) -> String {
        format!("{:#?}", self.driver)
    }

    // Spawn the model
    pub async fn spawn_model(&mut self) -> Result<(mpsc::Sender<String>, mpsc::Receiver<String>, mpsc::Receiver<String>), String> {
        self.driver.spawn_model().await
    }
}



// Unit tests
// Run with: cargo test -- --color always --nocapture
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::Utc;
    use std::collections::HashMap;

    // MEALArgs are comprised of a Vec of HashMaps (Depending on the driver type):
        //   - static_fields: HashMap<String, String>
        //   - model_params: HashMap<String, String>
        //   - connection_params: HashMap<String, String>

    #[tokio::test]
    async fn test_local_meal_1() {
        
        // Create testing MEALArgs for local MEAL using DialoGPT-small
        let mut meal_config: Vec<HashMap<String, String>> = Vec::new();
        // Create static_fields, model_params, and connection_params HashMaps
        let mut static_fields: HashMap<String, String> = HashMap::new();
        let mut model_params: HashMap<String, String> = HashMap::new();
        let mut connection_params: HashMap<String, String> = HashMap::new();

        // Add static_fields
        static_fields.insert("uid".to_string(), "1".to_string());
        static_fields.insert("createdAt".to_string(), Utc::now().to_string());
        static_fields.insert("lastUpdated".to_string(), Utc::now().to_string());
        static_fields.insert("name".to_string(), "DialoGPT-small".to_string());
        static_fields.insert("connType".to_string(), "local".to_string());

        // Get the current directory and move one layer up to the project root
        let mut current_dir = std::env::current_dir().unwrap();
        current_dir.pop();
        // Add the DialoGPT-small path to the current directory
        current_dir.push("test-models/local/DialoGPT-small");
        // Print the current directory
        println!("Current directory: {:#?}", current_dir);

        // Generate the command to run DialoGPT-small within a Python3 virtual environment
        let command = "conda activate transformer-venv && python3 inference.py".to_string();

        // Add model_params
        model_params.insert("modelPath".to_string(), current_dir.to_str().unwrap().to_string());
        model_params.insert("inferenceCommand".to_string(), command.to_string());
        model_params.insert("readyToken".to_string(), "@!#READY#!@".to_string());
        model_params.insert("exitToken".to_string(), "@!#EXIT#!@".to_string());
        model_params.insert("startToken".to_string(), "@!#START#!@".to_string());
        model_params.insert("stopToken".to_string(), "@!#STOP#!@".to_string());

        // Add connection_params
        connection_params.insert("createdAt".to_string(), Utc::now().to_string());
        connection_params.insert("lastUpdated".to_string(), Utc::now().to_string());

        // Add HashMaps to meal_config
        meal_config.push(static_fields);
        meal_config.push(connection_params);
        meal_config.push(model_params);

        // Create MEALArgs
        let meal_args = MEALArgs {
            meal_config: meal_config,
        };

        // Create MEAL
        let mut meal = MEAL::create("local", meal_args).unwrap();

        println!("MEAL: {:#?}", meal);

        // Spawn the model
        let (mut stdout_tx, mut stdout_rx, mut stderr_rx) = meal.spawn_model().await.unwrap();

        // Wait for the model to be ready
        println!("Waiting for the model to be ready...");
        let mut ready = false;
        while !ready {
            let stdout = stdout_rx.recv().await.unwrap();
            let stderr = stderr_rx.recv().await.unwrap();
            
            if stdout.contains("@!#READY#!@") {
                ready = true;
            } 

            if stdout.len() > 0 {
                println!("stdout: {:#?}", stdout);
            }

            if stderr.len() > 0 {
                println!("stderr: {:#?}", stderr);
            }
        }


        // Send a message to the model encapsulated in the start and stop tokens
        println!("Sending a message to the model...");
        let prompt = "Hello, how are you?";
        let prompt = "@!#START#!@\n".to_string() + prompt + "@!#STOP#!@\n";
        stdout_tx.send(prompt.clone()).await.unwrap();

        // Wait for the model to respond
        println!("Waiting for the model to respond...");
        let mut response = String::new();
        while !response.contains("@!#STOP#!@") {
            let stdout = stdout_rx.recv().await.unwrap();
            response = stdout;
        }

        // Remove the start and stop tokens from the response
        response = response.replace("@!#START#!@\n", "");
        response = response.replace("@!#STOP#!@\n", "");

        // Print the prompt and response
        println!("Prompt: {:#?}", prompt);
        println!("Response: {:#?}", response);

        // Send the exit token to the model
        println!("Sending the exit token to the model...");
        stdout_tx.send("@!#EXIT#!@".to_string()).await.unwrap();
    }
}