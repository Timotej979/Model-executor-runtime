// src/meal/mod.rs
use std::result::Result;
use async_trait::async_trait;
use std::collections::HashMap;

// Define MEALArgs struct
pub struct MEALArgs {
    pub meal_config: Vec<HashMap<String, String>>,
}

#[async_trait]
pub trait MEALDriver {
    
    // Create the MEALDriver constructor
    fn new(meal_args: MEALArgs) -> Self where Self: Sized;
    
    // MEALDriver methods
    async fn spawn_model(&mut self) -> Result<(), String>;

}

pub mod local;
pub mod ssh;

// MEAL struct
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

    // Add MEAL methods here

    pub async fn spawn_model(&mut self) -> Result<(), String> {
        self.driver.spawn_model().await
    }

}


// Unit tests
// Run with: cargo test -- --color always --nocapture
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // MEALArgs are comprised of a Vec of HashMaps (Depending on the driver type):
        //   - static_fields: HashMap<String, String>
        //   - model_params: HashMap<String, String>
        //   - connection_params: HashMap<String, String>

    #[tokio::test]
    async fn test_local_meal() {
        
        // Create testing MEALArgs for local MEAL using DialoGPT-small
        let mut meal_config: Vec<HashMap<String, String>> = Vec::new();
        // Create static_fields, model_params, and connection_params HashMaps
        let mut static_fields: HashMap<String, String> = HashMap::new();
        let mut model_params: HashMap<String, String> = HashMap::new();
        let mut connection_params: HashMap<String, String> = HashMap::new();

        // Add static_fields
        static_fields.insert("name".to_string(), "DialogGPT-small".to_string());
        static_fields.insert("connType".to_string(), "local".to_string());

        // Get the current directory and move one layer up to the project root
        let mut current_dir = std::env::current_dir().unwrap();
        current_dir.pop();
        current_dir.pop();
        // Add the DialoGPT-small path to the current directory
        current_dir.push("test-models/local/DialogGPT-small");

        // Print the current directory
        println!("DialogGPT-small path: {:#?}", current_dir);


    }
}