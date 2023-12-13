// /src/dal/surreal.rs
use super::{DatabaseDriver, DALArgs};
use async_trait::async_trait;
use std::collections::HashMap;

// SurrealDB
use once_cell::sync::Lazy;
use surrealdb::Surreal;
use surrealdb::sql::serde::serialize;
use surrealdb::sql::{Value, Object};
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
        log::info!("Connecting to the DB with url: {} and username: {}", self.connection_url, self.username);

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

    async fn get_available_models(&mut self) -> Result<Vec<String>, String> {
        log::info!("Getting available models from the DB...");

        //Create the available models vector
        let mut available_models: Vec<String> = Vec::new();

        // Get the available model UUIDs
        let ret: Result<Vec<String>, _> = self.db_conn
            .query("SELECT uid FROM AvailableModels")
            .await
            .map(|mut v| v.take("uid").unwrap());

        // Check if the query was successful
        if ret.is_err() {
            log::error!("Failed to get available models from the DB");
            return Err("Failed to get available models from the DB".to_string());
        }

        // Get the available model UUIDs and check if empty
        let available_model_uuids: Vec<String> = ret.unwrap();
        if available_model_uuids.is_empty() {
            log::error!("No available models found in the DB");
            return Err("No available models found in the DB".to_string());
        }

        
        // Loop over the available model UUIDs to get their properties
        for available_model_uuid in available_model_uuids {

            // Print the UID of the available model
            log::debug!("Available model UID: {:?}", available_model_uuid);

            // Get the available models staitic fields
            let mut ret: Result<(Vec<String>, Vec<String>, Vec<String>, Vec<String>), _> = self.db_conn
                .query("SELECT name, connType, createdAt, lastUpdated FROM AvailableModels WHERE uid = $uidString;")
                .bind(("uidString", available_model_uuid.clone()))
                .await
                .map(|mut v| (v.take("name").unwrap(), v.take("connType").unwrap(), v.take("createdAt").unwrap(), v.take("lastUpdated").unwrap()));

            if ret.is_err() {
                log::error!("Failed to get available models from the DB");
                return Err("Failed to get available models from the DB".to_string());
            }

            let (name, conn_type, created_at, last_updated) = ret.unwrap();

            log::debug!("\nAvailable model name: {:?}\n\
                         Available model connType: {:?}\n\
                         Available model createdAt: {:?}\n\
                         Available model lastUpdated: {:?}", name, conn_type, created_at, last_updated);


            // TODO: Bellow is not working, need to fix it

            // Get the available model connection params
            let available_model_connection_params: Result<Vec<(String, String)>, _> = self.db_conn
                .query("SELECT * FROM ConnTypeParams WHERE uid = $uidString;")
                .bind(("uidString", available_model_uuid.clone()))
                .await
                .map(|mut rows| {
                    rows.take(0).map_or_else(
                        |_| Vec::new(), // Return an empty Vec if there's no row
                        |row: Object| {
                            // Convert the row into a Vec<(String, String)>
                            row.columns()
                                .iter()
                                .map(|(key, value)| (key.to_string(), value.to_string()))
                                .collect()
                        }
                    )
                });


            if available_model_connection_params.is_err() {
                log::error!("Failed to get available models from the DB");
                return Err("Failed to get available models from the DB".to_string());
            }

            let available_model_connection_params: Vec<(String, String)> = available_model_connection_params.unwrap();

            log::debug!("Available model connection params: {:?}", available_model_connection_params);
            

            let mut available_model_model_params: surrealdb::Response = self.db_conn.query("
                SELECT * FROM ModelParams WHERE uid = $uidString;")
                .bind(("uidString", available_model_uuid.clone()))
                .await
                .map_err(|err| err.to_string())?;

            log::debug!("Available model model params: {:?}", available_model_model_params);

            let model_params: Vec<Value> = available_model_model_params.take(0).unwrap();
            

            // Create a vector of available model static fields with nested vectors of available model connection params and available model model params
            //let available_model: Vec<Vec<String>> = vec![available_model_static_fields, available_model_connection_params, available_model_model_params];

            // Print the available model
            //log::info!("Available model: {:?}", available_model);

            // Add the available model to the available models vector
            //available_models.push(available_model.to_string());
        }
        
        
        // Return the available models
        Ok(vec!["test".to_string()])
    }



}