// src/dal/mod.rs
use std::result::Result;
use async_trait::async_trait;
use std::collections::HashMap;

// Define DALArgs struct
pub struct DALArgs {
    pub connection_url: String,
    pub username: String,
    pub password: String,
}

// Create the DatabaseDriver trait, should be implemented by all DAL drivers
#[async_trait]
pub trait DatabaseDriver {

    // Create the DatabaseDriver constructor
    fn new(dal_args: DALArgs) -> Self where Self: Sized;

    // Create the DatabaseDriver connection methods
    async fn connect(&mut self) -> Result<(), String>;
    async fn disconnect(&mut self) -> Result<(), String>;
    
    // DatabaseDriver querry methods
    async fn get_available_models(&mut self) -> Result<Vec<Vec<HashMap<String, String>>>, String>;
}

// Re-export driver modules
pub mod surreal;

// DAL struct
pub struct DAL {
    driver: Box<dyn DatabaseDriver>,
}

impl DAL {
    pub fn create(driver_type: &str, dal_args: DALArgs) -> Result<Self, String> {
        let driver: Box<dyn DatabaseDriver> = match driver_type {
            "surreal" => Box::new(surreal::SurrealDriver::new(dal_args)),
            // Add other DAL drivers here, when implemented
            _ => {
                log::error!("Unknown DAL driver type: {}", driver_type);
                return Err("Unknown DAL driver type: ".to_string() + driver_type);
            }
        };

        Ok(Self { driver })
    }

    pub async fn connect(&mut self) -> Result<(), String> {
        self.driver.connect().await
    }

    pub async fn disconnect(&mut self) -> Result<(), String> {
        self.driver.disconnect().await
    }

    pub async fn get_available_models(&mut self) -> Result<Vec<Vec<HashMap<String, String>>>, String> {
        self.driver.get_available_models().await
    }

    // Add other DAL methods here
}


// Unit tests
// Run with: cargo test -- --color always --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    // Test the DAL create
    #[tokio::test]
    async fn test_dal_create() {
        // Create the DALArgs instance
        let dal_args = DALArgs {
            connection_url: "localhost:4321".to_string(),
            username: "driver".to_string(),
            password: "M0d3lDr1v3r".to_string(),
        };

        // Create the DAL instance
        let dal = DAL::create("surreal", dal_args);
        assert!(dal.is_ok());
        let _ = dal.unwrap();
    }

    // Test the DAL connect/disconnect
    #[tokio::test]
    async fn test_dal_connect_disconnect() {
        // Create the DALArgs instance
        let dal_args = DALArgs {
            connection_url: "localhost:4321".to_string(),
            username: "driver".to_string(),
            password: "M0d3lDr1v3r".to_string(),
        };

        // Create the DAL instance
        let mut dal = DAL::create("surreal", dal_args).unwrap();

        // Connect to the DAL
        let connect_result = dal.connect().await;
        assert!(connect_result.is_ok());

        // Disconnect from the DAL
        let disconnect_result = dal.disconnect().await;
        assert!(disconnect_result.is_ok());
    }

    // Test the DAL get_available_models
    #[tokio::test]
    async fn test_dal_get_available_models() {
        // Create the DALArgs instance
        let dal_args = DALArgs {
            connection_url: "localhost:4321".to_string(),
            username: "driver".to_string(),
            password: "M0d3lDr1v3r".to_string(),
        };

        // Create the DAL instance
        let mut dal = DAL::create("surreal", dal_args).unwrap();

        // Connect to the DAL
        let _ = dal.connect().await.expect("Failed to connect to the DAL");

        // Get the available models
        let available_models = dal.get_available_models().await.expect("Failed to get available models");

        // Print the available models
        for model in &available_models {
            println!("Model instance of {}:", model[0].get("name").expect("Failed to get model_type"));
            // Go over all 3 HashMaps in the Vec
            for model_instance in model {
                for (key, value) in model_instance {
                    println!("    - {}: {}", key, value);
                }
            }
            println!("-------------------------");
        }

        // Check if not empty
        assert!(available_models.len() > 0);
        assert!(available_models[0].len() > 0);

        // Check if static fields are present
        assert!(available_models[0][0].len() > 0);
        assert!(available_models[0][0].contains_key("uid"));
        assert!(available_models[0][0].contains_key("name"));
        assert!(available_models[0][0].contains_key("connType"));
        assert!(available_models[0][0].contains_key("createdAt"));
        assert!(available_models[0][0].contains_key("lastUpdated"));

        // Check if connection_params are present
        assert!(available_models[0][1].len() > 0);
        assert!(available_models[0][1].contains_key("uid"));
        assert!(available_models[0][1].contains_key("createdAt"));
        assert!(available_models[0][1].contains_key("lastUpdated"));

        // Check if model_params are present
        assert!(available_models[0][2].len() > 0);
        assert!(available_models[0][2].contains_key("uid"));
        assert!(available_models[0][2].contains_key("createdAt"));
        assert!(available_models[0][2].contains_key("lastUpdated"));

        // Disconnect from the DAL
        let _ = dal.disconnect().await.expect("Failed to disconnect from the DAL");
    }
}