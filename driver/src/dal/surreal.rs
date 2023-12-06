// General imports
use super::{DatabaseDriver, DALArgs};
use std::result::Result;
use async_trait::async_trait;

// SurrealDB
use once_cell::sync::Lazy;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws, Wss};
use surrealdb::opt::auth::Root as surrealRoot;



// Create the SurrealDriver struct
pub struct SurrealDriver {
    connection_url: String,
    username: String,
    password: String,
    namespace: String,
    database: String,
    db_conn: Lazy::new(Surreal::init),
}

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
            let _ = &self.db_conn.connect::<Ws>(&self.connection_url).await?;
        } else {
            // Use wss
            let _ = &self.db_conn.connect::<Wss>(&self.connection_url).await?;
        }

        // Sign in as user
        let _ = &self.db_conn.signin(surrealRoot {
            username: &self.username,
            password: &self.password,
        })
        .await?;

        // Use the namespace and database
        let _ = &self.db_conn.use_ns(&self.namespace).use_db(&self.database).await?;

        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), String> {
        log::info!("Disconnecting from the DB...");
        // Disconnect from the DB with invalidating the connection
        let _ = &self.db_conn.invalidate().await?;
        Ok(())
    }


    ///////////////////////////////////////////////////
    ///// Management of the SurrealDriver queries /////
    ///////////////////////////////////////////////////

}