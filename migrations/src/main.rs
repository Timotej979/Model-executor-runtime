// Logging
use chrono::prelude::{DateTime, Utc};
use std::sync::{Arc};
use ansi_term::Colour;
use std::time::SystemTime;
use logging::{Handler, Level, Message};

// SurrealDB
use once_cell::sync::Lazy;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws, Wss};
use surrealdb::opt::auth::Root as surrealRoot;



///////////////////////////////////////////////////////////////////////////////////////
// Initialize the custom logger format for logging crate
pub struct PublicMessage {
    /// The level of the message.
    level: Level,
    /// The full name of the logger.
    name: String,
    /// The timestamp of creation.
    created: SystemTime,
    /// The message string itself.
    msg: String,
}

struct CustomLoggingHandler {}

impl Handler for CustomLoggingHandler {
    fn emit(&self, msg: &Message) {
        // Create a public message with the same data
        let public_msg = unsafe { std::mem::transmute::<&Message, &PublicMessage>(msg) };

        let timestamp: DateTime<Utc> = public_msg.created.clone().into();

        // Parse the level to human-readable format
        let data = match public_msg.level {
            Level::DEBUG => Colour::Cyan.paint(format!("{} | DEBUG [{:15}] {}\n", timestamp, public_msg.name, public_msg.msg)),
            Level::INFO => Colour::Green.paint(format!("{} | INFO [{:15}] {}\n", timestamp, public_msg.name, public_msg.msg)),
            Level::WARN => Colour::Yellow.paint(format!("{} | WARN [{:15}] {}\n", timestamp, public_msg.name, public_msg.msg)),
            Level::ERROR => Colour::Red.paint(format!("{} | ERROR [{:15}] {}\n", timestamp, public_msg.name, public_msg.msg)),
            Level::FATAL => Colour::Red.bold().paint(format!("{} | FATAL [{:15}] {}\n", timestamp, public_msg.name, public_msg.msg)),
            Level::NONE => Colour::White.paint(""),
        };

        print!("{}", data);
    }
}

// Initialize the loggers
struct InitLoggers {
    prefixes: Vec<String>
}

impl InitLoggers {
    fn new(prefixes: Vec<String>) -> Self {
        Self {
            prefixes
        }
    }

    fn build(&self) -> Vec<logging::Logger> {
        // Initialize the logger
        logging::root().add_handler(Arc::new(Box::new(CustomLoggingHandler {})));

        // Initialize the loggers vector
        let mut loggers = Vec::new();

        // Iterate through the prefixes
        for prefix in &self.prefixes {
            // Create a new logger
            let log = logging::get(prefix);

            // Push the logger to the loggers vector
            loggers.push(log);
        }

        // Return the loggers vector
        return loggers;
    }
}



///////////////////////////////////////////////////////////////////////////////////////
// MigrationManager

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
        let _ = &self.db_conn.signin(surrealRoot {
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

    // Log the start of the migration
    logging::info("Starting the migration");

    // TODO: Parse the CLI arguments with clap

    // Set the prefixes
    let prefixes: Vec<String> = vec![
        "migrations::main".to_string(),
        "migrations::migration_manager".to_string()
    ];

    // Initialize the loggers
    let loggers = InitLoggers::new(prefixes).build();

    let main_logger = &loggers[0];
    let migration_manager_logger = &loggers[1];

    main_logger.info("Initialized the loggers");
    main_logger.debug("Initialized the loggers");
    main_logger.warn("Initialized the loggers");
    main_logger.error("Initialized the loggers");
    main_logger.fatal("Initialized the loggers");

    migration_manager_logger.info("Initialized the loggers");
    migration_manager_logger.debug("Initialized the loggers");
    migration_manager_logger.warn("Initialized the loggers");
    migration_manager_logger.error("Initialized the loggers");
    migration_manager_logger.fatal("Initialized the loggers");
    




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
