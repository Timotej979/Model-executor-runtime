// /src/dal/surreal.rs
use super::{DatabaseDriver, DALArgs};
use std::result::Result;
use async_trait::async_trait;

// SurrealDB
use once_cell::sync::Lazy;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws, Wss};
use surrealdb::opt::auth::Root as surrealRoot;

// Serialize and deserialize
use serde_json::Value;


// General DB response struct
struct DBResponse {
    status: String,
    data: Value,
}

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
            let _ = &self.db_conn.connect::<Ws>(&self.connection_url).await.expect("Failed to connect to the DB");
        } else {
            // Use wss
            let _ = &self.db_conn.connect::<Wss>(&self.connection_url).await.expect("Failed to connect to the DB");
        }

        // Sign in as user
        let _ = &self.db_conn.signin(surrealRoot {
            username: &self.username,
            password: &self.password,
        })
        .await.expect("Failed to sign in to the DB");

        // Use the namespace and database
        let _ = &self.db_conn.use_ns(&self.namespace).use_db(&self.database).await.expect("Failed to use the namespace and database");

        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), String> {
        log::info!("Disconnecting from the DB...");
        // Disconnect from the DB with invalidating the connection
        let _ = &self.db_conn.invalidate().await.expect("Failed to disconnect from the DB");
        Ok(())
    }


    ///////////////////////////////////////////////////
    ///// Management of the SurrealDriver queries /////
    ///////////////////////////////////////////////////

    async fn get_available_models(&mut self) -> Result<Vec<String>, String> {
        log::info!("Getting available models from the DB...");

        //Create the available models vector
        let mut available_models: Vec<String> = Vec::new();

        // Get the available model UIDs
        let available_model_uids = &self.db_conn.query("
            SELECT uid FROM AvailableModels;
        ").await.expect("Failed to get available models");

        log::info!("Available model UIDs: {:?}", available_model_uids);

        // Deserialize the available model UIDs response
        // Deserialize the JSON response into the DBResponse struct
        //let result = serde_json::from_str(available_model_uids);


        // Parse the available model UIDs
        /*
        for available_model_uid in available_model_uids {

            // Print the uuid
            log::info!("Available model UID: {:?}", available_model_uid);

            // Get the available models staitic fields
            let available_model_static_fields = &self.db_conn.query("
                SELECT name, connType, createdAt, lastUpdated FROM AvailableModels WHERE uid = type::is::uuid($uidString);")
                .bind(("uidString", available_model_uid))
                .await
                .expect("Failed to get available model static fields");

            // Get the available model connection params
            let available_model_connection_params = &self.db_conn.query("
                SELECT * FROM ConnTypeParams WHERE uid = type::is::uuid($uidString);")
                .bind(("uidString", available_model_uid))
                .await
                .expect("Failed to get available model connection params");

            let available_model_model_params = &self.db_conn.query("
                SELECT * FROM ModelParams WHERE uid = type::is::uuid($uidString);")
                .bind(("uidString", available_model_uid))
                .await
                .expect("Failed to get available model model params");

            // Create a vector of available model static fields with nested vectors of available model connection params and available model model params
            let available_model: Vec<Vec<String>> = vec![available_model_static_fields, available_model_connection_params, available_model_model_params];

            // Print the available model
            log::info!("Available model: {:?}", available_model);

            // Add the available model to the available models vector
            available_models.push(available_model.to_string());
        }
        */
        
        // Return the available models
        Ok(vec!["test".to_string()])
    }



}