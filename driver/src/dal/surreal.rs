// /src/dal/surreal.rs
use super::{DatabaseDriver, DALArgs};
use async_trait::async_trait;
use std::collections::HashMap;

// SurrealDB
use once_cell::sync::Lazy;
use surrealdb::Surreal;
use surrealdb::sql::Value;
use surrealdb::engine::remote::ws::{Client, Ws, Wss};
use surrealdb::opt::auth::Root as surrealRoot;


// Create the SurrealDriver struct
pub struct SurrealDriver {
    connection_url: String,
    username: String,
    password: String,
    namespace: String,
    database: String,
    db_conn: Lazy<Surreal<Client>>,
}

#[async_trait]
impl DatabaseDriver for SurrealDriver {

    ////////////////////////////////////////////////////
    ///// Management of the SurrealDriver instance /////
    ////////////////////////////////////////////////////

    fn new(dal_args: DALArgs) -> Self {
        Self {
            connection_url: dal_args.connection_url,
            username: dal_args.username,
            password: dal_args.password,
            namespace: "ModelExecutorRuntimeNS".to_string(),
            database: "ModelExecutorRuntimeDB".to_string(),
            db_conn: Lazy::new(Surreal::init),
        }
    }


    //////////////////////////////////////////////////////
    ///// Management of the SurrealDriver connection /////
    //////////////////////////////////////////////////////

    async fn connect(&mut self) -> Result<(), String> {
        log::info!("Connecting to the DB with url: {:#?} and username: {:#?}", self.connection_url, self.username);

        // Check if the connection URL includes localhost
        if self.connection_url.starts_with("localhost:") || self.connection_url.starts_with("127.0.0.1:") || self.connection_url.starts_with("0.0.0.0:") {
            // Use ws
            let _ = &self.db_conn.connect::<Ws>(&self.connection_url).await.map_err(|err| err.to_string())?;
        } else {
            // Use wss
            let _ = &self.db_conn.connect::<Wss>(&self.connection_url).await.map_err(|err| err.to_string())?;
        }

        // Sign in as user
        let _ = &self.db_conn.signin(surrealRoot {
            username: &self.username,
            password: &self.password,
        })
        .await.map_err(|err| err.to_string())?;

        // Use the namespace and database
        let _ = &self.db_conn.use_ns(&self.namespace).use_db(&self.database).await.map_err(|err| err.to_string())?;

        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), String> {
        log::info!("Disconnecting from the DB...");
        // Disconnect from the DB with invalidating the connection
        let _ = &self.db_conn.invalidate().await.map_err(|err| err.to_string())?;
        Ok(())
    }


    ///////////////////////////////////////////////////
    ///// Management of the SurrealDriver queries /////
    ///////////////////////////////////////////////////

    async fn get_available_models(&mut self) -> Result<Vec<Vec<HashMap<String, String>>>, String> {
        log::info!("Getting available models from the DB...");

        // Create the available models HashMap to return
        let mut available_models: Vec<Vec<HashMap<String, String>>> = Vec::new();


        // Get the available model UUIDs
        let result: Result<Vec<String>, _> = self.db_conn
            .query("SELECT uid FROM AvailableModels")
            .await
            .map(|mut v| v.take("uid").unwrap());

        if result.is_err() {
            log::error!("Failed to get available model UUIDs from the DB");
            return Err("Failed to get available model UUIDs from the DB".to_string());
        }
        // Get the available model UUIDs and check if empty
        let available_model_uuids: Vec<String> = result.unwrap();
        if available_model_uuids.is_empty() {
            log::error!("No available models found in the DB");
            return Err("No available models found in the DB".to_string());
        }

        
        // Loop over the available model UUIDs to get their properties
        for available_model_uuid in available_model_uuids {
            log::debug!("Processing model with UUID: {:#?}", available_model_uuid);


            // Get the available models staitic fields
            let result: Result<Value, _> = self.db_conn
                .query("SELECT name, connType, createdAt, lastUpdated FROM AvailableModels WHERE uid = $uidString;")
                .bind(("uidString", available_model_uuid.clone()))
                .await
                .map(|mut v| v.take(0).unwrap());

            if result.is_err() {
                log::error!("Failed to get static fields for available models from the DB");
                return Err("Failed to get static fields for available models from the DB".to_string());
            }
            // Parse the available model static fields into a HashMap
            let result_json = result.unwrap().into_json();
            let mut static_fields = HashMap::new();
            if let Some(first_element) = result_json.as_array().and_then(|arr| arr.get(0)) {
                // Convert JsonValues to Strings or other appropriate types if needed
                static_fields = first_element
                    .as_object()
                    .expect("Expected object in the array")
                    .iter()
                    .map(|(key, value)| (key.clone(), value.as_str().unwrap_or_default().to_string()))
                    .collect();
                log::debug!("    - Static fields processed succesfully");
            }
            // Add the available model UUID to the available model static fields HashMap
            static_fields.insert("uid".to_string(), available_model_uuid.clone());



            // Get the available model connection params
            let result: Result<Value, _> = self.db_conn
                .query("SELECT * FROM ConnTypeParams WHERE uid = $uidString;")
                .bind(("uidString", available_model_uuid.clone()))
                .await
                .map(|mut v| v.take(0).unwrap());

            if result.is_err() {
                log::error!("Failed to get connection params for available models from the DB");
                return Err("Failed to get connection params for available models from the DB".to_string());
            }
            // Parse the available model connection params into a HashMap
            let result_json = result.unwrap().into_json();
            let mut connection_params = HashMap::new();
            if let Some(first_element) = result_json.as_array().and_then(|arr| arr.get(0)) {
                // Convert JsonValues to Strings or other appropriate types if needed
                connection_params = first_element
                    .as_object()
                    .expect("Expected object in the array")
                    .iter()
                    .map(|(key, value)| (key.clone(), value.as_str().unwrap_or_default().to_string()))
                    .collect();
                log::debug!("    - Connection params processed succesfully");
            }
            

            // Get the available model model params
            let result: Result<Value, _> = self.db_conn
                .query("SELECT * FROM ModelParams WHERE uid = $uidString;")
                .bind(("uidString", available_model_uuid.clone()))
                .await
                .map(|mut v| v.take(0).unwrap());

            if result.is_err() {
                log::error!("Failed to get model params for available models from the DB");
                return Err("Failed to get model params for available models from the DB".to_string());
            }

            // Parse the available model model params into a HashMap
            let result_json = result.unwrap().into_json();
            let mut model_params = HashMap::new();
            if let Some(first_element) = result_json.as_array().and_then(|arr| arr.get(0)) {
                // Convert JsonValues to Strings or other appropriate types if needed
                model_params = first_element
                    .as_object()
                    .expect("Expected object in the array")
                    .iter()
                    .map(|(key, value)| (key.clone(), value.as_str().unwrap_or_default().to_string()))
                    .collect();
                log::debug!("    - Model params for uid processed succesfully");
            }

            
            // Insert the available model static fields, connection params and model params into a vector
            let mut available_model = Vec::new();
            available_model.push(static_fields);
            available_model.push(connection_params);
            available_model.push(model_params);

            // Insert the available model into the available models vector
            available_models.push(available_model);
        }
        
        // Return the available models
        Ok(available_models)
    }



}