// src/meal/ssh.rs
use super::{MEALDriver, MEALArgs};
use async_trait::async_trait;
use std::collections::HashMap;


// Create the SSHDriver struct
pub struct SSHDriver {
    static_fields: HashMap<String, String>,
    model_params: HashMap<String, String>,
    connection_params: HashMap<String, String>,
}

#[async_trait]
impl MEALDriver for SSHDriver {

    ////////////////////////////////////////////////////
    ///// Management of the SSHDriver instance /////
    ////////////////////////////////////////////////////

    fn new(meal_args: MEALArgs) -> Self {
        Self {
            static_fields: meal_args.meal_config[0].clone(),
            model_params: meal_args.meal_config[1].clone(),
            connection_params: meal_args.meal_config[2].clone(),
        }
    }

    //////////////////////////////////////////////////////
    /////// Management of the SSHDriver connection ///////
    //////////////////////////////////////////////////////
    async fn spawn_model(&self) -> Result<tokio::process::Child, String> {

    
        Ok()
    }

}