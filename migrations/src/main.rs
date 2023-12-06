// Logging with env_logger
use env_logger;
use log;

// CLI arg parsing with clap
use clap::Parser;

// File reading and regex
use regex::Regex;
use std::fs;

// SurrealDB
use once_cell::sync::Lazy;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws, Wss};
use surrealdb::opt::auth::Root as surrealRoot;



///////////////////////////////////////////////////////////////////////////////////////
// Parse CLI args using the clap crate with the Derive API
#[derive(Parser)]
#[command(name = "Model-Executor-Runtime-Migrations (MER-Migrations)")]
#[command(author = "Timotej979 @ GitHub")]
#[command(version = "1.0")]
#[command(about = "Migrates the required DB schema for running the MER", long_about = None)]
struct Cli {
    #[arg(short, long, env = "DB_CONNECTION_URL", default_value = "localhost:4321")]
    connection_url: String,
    
    #[arg(short, long, env = "MIGRATIONS_DB_USERNAME", default_value = "driver")]
    username: String,
    
    #[arg(short, long, env = "MIGRATIONS_DB_PASSWORD", default_value = "M0d3lDr1v3r")]
    password: String,
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
    migration_file_regex: Regex,
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
            migration_file_regex: Regex::new(r"^(?P<number>\d{2})-(?P<name>\w+)\.sql$").expect("Failed to compile the migration file regex"),
        })
    }

    // Connect to DB
    async fn connect(&mut self) -> surrealdb::Result<()> {

        // Log connection
        log::info!("Connecting to DB with url {} and username {}", self.connection_url, self.username);

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

        Ok(())
    }

    // Check if the migrations table exists
    async fn check_migrations_table_exists(&mut self) -> surrealdb::Result<surrealdb::Response> {
        let response = self.db_conn.query("SELECT * FROM Migrations ORDER BY num DESC LIMIT 1;").await;
        return response;
    }

    // Create the migrations table
    async fn create_migrations_table(&mut self) -> surrealdb::Result<surrealdb::Response> {
        let response = self.db_conn.query("
            DEFINE TABLE Migrations SCHEMALESS;
            DEFINE FIELD num ON TABLE Migrations TYPE number ASSERT $value != NONE AND $value != NULL; 
            DEFINE FIELD operation ON TABLE Migrations TYPE string ASSERT $value != NONE AND $value != NULL;
            DEFINE FIELD timestamp ON TABLE Migrations TYPE datetime ASSERT $value != NONE AND $value != NULL;
        ").await;
        return response;
    }

    // Migrate the DB
    async fn migrate(&mut self) -> surrealdb::Result<()> {
        // Check if migration table exists
        let mut migrations_exist = self.check_migrations_table_exists().await?;

        // Get the num column
        let mut num: Vec<i64> = migrations_exist.take("num")?;

        // If the migrations table does not exist
        if num.is_empty()  {
            // Create the migrations table
            let _ = self.create_migrations_table().await?;
            // Append 0 to the num vector
            num.push(0);
        }
        
        // Read the migrations directory with error checks
        if let Ok(entries) = fs::read_dir("./sql") {
            // Create a vector to hold the migration files
            let mut migration_files: Vec<(i64, String, String)> = Vec::new();

            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        // Check if the file name matches the regex
                        if let Some(captures) = self.migration_file_regex.captures(file_name) {
                            // Get the migration number
                            let migration_number = captures.name("number").unwrap().as_str().parse::<i64>().unwrap();
                            // Check if the migration has already been executed
                            if num[0] >= migration_number {
                                log::info!("Migration {} has already been executed", migration_number);
                                continue;
                            }

                            // Get the migration name
                            let migration_name = captures.name("name").unwrap().as_str();

                            // Read the file
                            let migration_file = fs::read_to_string(entry.path()).expect("Failed to read the migration file");

                            if migration_file.is_empty() {
                                log::error!("Migration file: {:?} is empty, not executing it.", entry);
                                continue;
                            } else {
                                log::info!("Read the migration file: {:?}", entry);
                                // Add the migration to the list to execute
                                migration_files.push((migration_number, migration_name.to_string(), migration_file));
                            }
                        } else {
                            log::error!("Failed to parse the migration file name: {:?}", entry);
                        }
                    }
                } else {
                    log::error!("Failed to read the migration: {:?}", entry);
                }
            }

            // Sort migration files by number
            migration_files.sort_by(|a, b| a.0.cmp(&b.0));

            // Execute migrations consecutively
            for (migration_number, migration_name, migration_file) in migration_files {
                log::info!("Executing migration {}...", migration_number);
                // Execute the migration
                let _ = self.db_conn.query(&migration_file).await?;
                // Insert the migration into the migrations table
                let _ = self.db_conn.query(&format!("INSERT INTO Migrations (num, operation, timestamp) VALUES ({}, '{}', time::now());", migration_number, migration_name)).await?;
            
                log::info!("Executed migration {}", migration_number);
            }
        } else {
            log::error!("Failed to read the migrations directory");
        }

        // Return
        Ok(())
    }

    // Disconnect from DB
    async fn disconnect(&mut self) -> surrealdb::Result<()> {
        // Invalidate the current connection
        let _ = &self.db_conn.invalidate().await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {

    // Initialize the logger
    log::info!("Initializing the logger...");
    env_logger::init();

    // Parse command line arguments
    log::info!("Parsing CLI args...");
    let args = Cli::parse();

    // Create a new MigrationManager
    log::info!("Creating new MigrationManager...");
    let mut manager = MigrationManager::new(
        // Connection URL
        args.connection_url,
        // Username
        args.username,
        // Password
        args.password,
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
