// Logging with env_logger
use env_logger;

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
#[command(name = "Model-executor-runtime-Migrations (MER-Migrations)")]
#[command(author = "Timotej979 @ GitHub")]
#[command(version = "1.0")]
#[command(about = "Migrates the required DB schema for running the MER", long_about = None)]
struct Cli {
    #[arg(short, long, env = "MIGRATIONS_CONNECTION_URL", default_value = "localhost:4321")]
    connection_url: String,
    
    #[arg(short, long, env = "MIGRATIONS_USERNAME", default_value = "driver")]
    username: String,
    
    #[arg(short, long, env = "MIGRATIONS_PASSWORD", default_value = "M0d3lDr1v3r")]
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

        // Return
        Ok(())
    }

    // Migrate the DB
    async fn migrate(&mut self) -> surrealdb::Result<()> {
        // Check if migration table exists
        let migrations_exist = &mut self.db_conn.query("SELECT num FROM Migrations ORDER BY num DESC LIMIT 1 ;").await?;
        
        let num: Vec<String> = migrations_exist.take("num")?;

        if num.is_empty() {
            let _ = &self.db_conn.query("
                    DEFINE TABLE Migrations SCHEMALESS;
                    DEFINE FIELD num ON TABLE Migrations TYPE number ASSERT $value != NONE AND $value != NULL; 
                    DEFINE FIELD operation ON TABLE Migrations TYPE string ASSERT $value != NONE AND $value != NULL;
                    DEFINE FIELD timestamp ON TABLE Migrations TYPE datetime ASSERT $value != NONE AND $value != NULL;
                ").await?;
        }

        // Get the number of the last migration
        let last_migration_num = num[0].parse::<i64>().unwrap();

        // Read the migrations directory
        let dir = std::fs::read_dir("./sql");
        /*
        // Iterate through the migrations
        for entry in dir {
            if Ok(entry) = entry {
                let Some(file_name) = entry.file_name.to_string() {
                    if migration_file_regex.is_match(filename){
                        if let Some(captures) = migration_file_regex.captures(file_name) {
                            if let Some(number) = captures.name("number") {
                                if let Some(name) = captures.name("name") {
                                    if let (Ok(parsed_number), Some(parsed_name)) = (number.as_str().parse::<i64>(), name.as_str()) {
                                        println!(
                                            "Number: {}, Name: {}",
                                            parsed_number, parsed_name
                                        );
                                    }                
                                }
                            }   
                        }
                    }
                }
            }
        }
        */



        
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
