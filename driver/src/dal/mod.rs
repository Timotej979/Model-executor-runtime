// dal/mod.rs
use std::result::Result;
use async_trait::async_trait;

// Define DALArgs struct
pub struct DALArgs {
    pub connection_url: String,
    pub username: String,
    pub password: String,
}

// Create the DatabaseDriver trait, should be implemented by all DAL drivers
#[async_trait]
pub trait DatabaseDriver {
    fn new(dal_args: DALArgs) -> Self;

    async fn connect(&mut self) -> Result<(), String>;
    async fn disconnect(&mut self) -> Result<(), String>;
    // Add other DAL methods here
}

// Re-export driver modules
pub mod surreal;

// DAL struct
pub struct DAL<T: DatabaseDriver> {
    driver: T,
}

impl<T: DatabaseDriver> DAL<T> {
    pub fn create(driver_type: &str, dal_args: DALArgs) -> Result<Self, String> {
        let driver: T = match driver_type {
            "surreal" => surreal::SurrealDriver::new(dal_args),
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

    // Add other DAL methods here
}