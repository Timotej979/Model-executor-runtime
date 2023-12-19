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

}