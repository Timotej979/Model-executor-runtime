// Logging
use env_logger::{ Builder, Logger};
use log::LevelFilter;

// SurrealDB
use once_cell::sync::Lazy;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws, Wss};
use surrealdb::opt::auth::Root;


struct LoggerManager {
    loggers: Vec<Logger>,
}

impl LoggerManager {
    fn new(logger_prefixes: &[&str], log_coloring: bool) -> Self {
        let mut loggers = Vec::new();

        for logger_prefix in logger_prefixes {
            let mut builder = Builder::from_default_env();

            if log_coloring {
                builder.write_style(env_logger::WriteStyle::Always);
            }

            builder.filter_module(logger_prefix, LevelFilter::Debug);
            let logger = builder.build();

            loggers.push(logger);
        }

        Self { loggers }
    }

    fn get_loggers(&self) -> &[Logger] {
        &self.loggers
    }
}



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
        if self.connection_url.starts_with("localhost:") || self.connection_url.starts_with("127.0.0.1:") || self.connection_url.starts_with("0.0.0.0:") {
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
        let migrations_exist = &mut self.db_conn.query("SELECT num FROM Migrations ORDER BY num DESC LIMIT 1 ;").await?;
        
        let num: Vec<String> = migrations_exist.take("num")?;


        if num.is_empty() {
            // Create the migrations table
            let _ = &self.db_conn.query("
                DEFINE TABLE Migrations SCHEMALESS;
                DEFINE FIELD num ON TABLE Migrations TYPE number ASSERT $value != NONE AND $value != NULL; 
                DEFINE FIELD operation ON TABLE Migrations TYPE string ASSERT $value != NONE AND $value != NULL;
                DEFINE FIELD timestamp ON TABLE Migrations TYPE datetime ASSERT $value != NONE AND $value != NULL;
            ").await?;
        }

        
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
    // Create a new LoggerManager
    let logger_prefixes = ["MAIN", "DAL", "SCHEDULER"];
    let loggers = LoggerManager::new(&logger_prefixes, true);

    for logger in loggers.get_loggers() {
        logger.debug("This is a debug log");
        logger.warn("This is a warn log");
        logger.info("This is an info log");
        logger.error("This is an error log");
    }

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
