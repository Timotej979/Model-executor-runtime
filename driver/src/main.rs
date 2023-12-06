// Logging with env_logger
use env_logger;
use log;

// CLI parsing
use clap::Parser;

// Custom modules
mod repl;
mod dal;


///////////////////////////////////////////////////////////////////////////////////////
// Parse the main CLI args using the clap crate with the Derive API
#[derive(Parser)]
#[command(name = "Model-executor-runtime-Driver (MER-Driver)")]
#[command(author = "Timotej979 @ GitHub")]
#[command(version = "1.0")]
#[command(about = "Runs the driver for concurent model execution", long_about = None)]
struct Cli {
    #[arg(short, long, env = "DB_CONNECTION_URL", default_value = "localhost:4321")]
    connection_url: String,
    
    #[arg(short, long, env = "DRIVER_DB_USERNAME", default_value = "driver")]
    username: String,
    
    #[arg(short, long, env = "DRIVER_DB_PASSWORD", default_value = "M0d3lDr1v3r")]
    password: String,

    #[arg(short, long, env = "ALLOW_MODEL_SERVER_RUNTIME_CHANGES", default_value = "false", help = "Allow runtime changes to the model server DB")]
    allow_model_server_runtime_changes: bool,
}


///////////////////////////////////////////////////////////////////////////////////////
#[tokio::main]
async fn main() {

    // Initialize the logger
    log::info!("Initializing the logger...");
    env_logger::init();

    // Parse command line arguments
    log::info!("Parsing CLI args...");
    let args = Cli::parse();

    // Print the parsed arguments
    log::info!("Parsed CLI args:");
    log::info!("    - connection_url: {}", args.connection_url);
    log::info!("    - username: {}", args.username);
    log::info!("    - allow_model_server_runtime_changes: {}", args.allow_model_server_runtime_changes);


    log::info!("Initializing the DAL...");
    // Create the DALArgs instance
    let dal_args = dal::DALArgs {
        connection_url: args.connection_url,
        username: args.username,
        password: args.password,
    };

    // Create the DAL instance, currently only surreal is supported
    // If there is a need for more drivers, implement the driver and make the variable friver_type CLI parsed
    let driver_type = "surreal".to_string();

    let mut dal_instance = match dal::DAL::create(&driver_type, dal_args) {
        Ok(instance) => instance,
        Err(error) => {
            log::error!("Failed to create the DAL instance: {}", error);
            std::process::exit(1);
        }
    };

    log::info!("Connecting to the DAL...");
    // Connect to the DAL
    let _ = dal_instance.connect().await.expect("Failed to connect to the DAL");

    // Disconnect from the DAL
    let _ = dal_instance.disconnect().await.expect("Failed to disconnect from the DAL");




    // Initialize other structs

    ///////////////////////////////////////////////////////////////////////////////////////


    ///////////////////////////////////////////////////////////////////////////////////////
    // Start the CLI loop
    log::info!("Starting the CLI loop...");

    // Get the STD pipes
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let stderr = std::io::stderr();

    // Initialize the CliReplManager
    let mut crm_instance = repl::CliReplManager::new(stdin, stdout, stderr,
                                                     args.allow_model_server_runtime_changes)
                                                     .expect("Failed to initialize the CliReplManager");

    // Start the REPL
    let _ = crm_instance.repl();

}