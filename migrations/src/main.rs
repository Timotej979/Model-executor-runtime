use once_cell::sync::Lazy;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws, Wss};
use surrealdb::opt::auth::Root;


struct MigrationManager {
    connection_url: String,
    username: String,
    password: String,
    namespace: String,
    database: String,
    db_conn: Lazy<Surreal<Client>>,
}

impl MigrationManager {
    fn new(
        connection_url: String,
        username: String,
        password: String,
        namespace: String,
        database: String,
    ) -> surrealdb::Result<Self> {
        Ok(Self {
            connection_url,
            username,
            password,
            namespace,
            database,
            db_conn: Lazy::new(Surreal::init),
        })
    }

    async fn connect(&mut self) -> surrealdb::Result<()> {

        // Check if the connection URL includes localhost
        if self.connection_url.contains("localhost") {
            // Use ws
            let _ = &self.db_conn.connect::<Ws>(&self.connection_url).await?;
        } else {
            // Use wss
            let _ = &self.db_conn.connect::<Wss>(&self.connection_url).await?;
        }

        // Sign in as root
        let _ = &self.db_conn.signin(Root {
            username: &self.username,
            password: &self.password,
        })
        .await?;

        // Use the namespace and database
        let _ = &self.db_conn.use_ns(&self.namespace).use_db(&self.database).await?;

        // Return
        Ok(())
    }

    async fn migrate(&mut self) -> surrealdb::Result<()> {
        
        // Check if migration table exists
        let migrations_exist = &self.db_conn.query("SELECT * FROM Migrations;").await?;
        dbg!(migrations_exist);

        
        
        // TODO: Parse the table_exists to check if the table exists


        // Return
        Ok(())
    }

    async fn disconnect(&mut self) -> surrealdb::Result<()> {
        
        // Disconnect from the database
        dbg!("Disconnect");

        // Return
        Ok(())
    }
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Create a new MigrationManager
    let mut manager = MigrationManager::new(
        // Connection URL
        "localhost:4321".to_string(),
        // Username
        "driver".to_string(),
        // Password
        "M0d3lDr1v3r".to_string(),
        // Namespace
        "ModelExecutorRuntimeNS".to_string(),
        // Database
        "ModelExecutorRuntimeDB".to_string(),
    )?;

    // Connect to the database
    manager.connect().await?;

    // Migrate
    manager.migrate().await?;

    // Disconnect from the database
    manager.disconnect().await?;

    Ok(())
}
