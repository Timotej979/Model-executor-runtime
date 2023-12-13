// /src/dal/surreal.rs
use super::{DatabaseDriver, DALArgs};
use async_trait::async_trait;

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

        // Send the query to the DB
        let mut response: surrealdb::Response = self.db_conn
            .query("SELECT uid FROM AvailableModels;")
            .await
            .map_err(|err| err.to_string())?;

        // Get the available model UIDs and check if empty
        let available_model_uids: Vec<String> = response.take("uid").unwrap();
        if available_model_uids.is_empty() {
            log::error!("No available models found in the DB");
            return Err("No available models found in the DB".to_string());
        }

        // Loop over the available model UIDs to get their properties
        for available_model_uid in available_model_uids {

            // Print the UID of the available model
            log::debug!("Available model UID: {:?}", available_model_uid);

            // Get the available models staitic fields
            let mut available_model_static_fields: surrealdb::Response = self.db_conn.query("
                SELECT name, connType, createdAt, lastUpdated FROM AvailableModels WHERE uid = $uidString;")
                .bind(("uidString", available_model_uid.clone()))
                .await
                .map_err(|err| err.to_string())?;

            let name: Vec<String> = available_model_static_fields.take("name").unwrap();
            let conn_type: Vec<String> = available_model_static_fields.take("connType").unwrap();
            let created_at: Vec<String> = available_model_static_fields.take("createdAt").unwrap();
            let last_updated: Vec<String> = available_model_static_fields.take("lastUpdated").unwrap();

            log::debug!("\nAvailable model name: {:?}\n\
                         Available model connType: {:?}\n\
                         Available model createdAt: {:?}\n\
                         Available model lastUpdated: {:?}", name, conn_type, created_at, last_updated);


            // TODO: Bellow is not working, need to fix it

            // Get the available model connection params
            let mut available_model_connection_params: surrealdb::Response = self.db_conn.query("
                SELECT * FROM ConnTypeParams WHERE uid = $uidString;")
                .bind(("uidString", available_model_uid.clone()))
                .await
                .map_err(|err| err.to_string())?;

            log::debug!("Available model connection params: {:?}", available_model_connection_params);

            let connection_params: Vec<(String, String)> = available_model_connection_params.take(0).unwrap();

            // Print the available model connection params
            log::info!("Available model connection params: {:?}", connection_params);

            let mut available_model_model_params: surrealdb::Response = self.db_conn.query("
                SELECT * FROM ModelParams WHERE uid = $uidString;")
                .bind(("uidString", available_model_uid.clone()))
                .await
                .map_err(|err| err.to_string())?;

            log::debug!("Available model model params: {:?}", available_model_model_params);

            let model_params: Vec<String> = available_model_model_params.take(0).unwrap();

            // Print the available model model params
            log::info!("Available model model params: {:?}", model_params);
            

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