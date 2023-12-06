// Logging with env_logger
use env_logger;
use log;

// CLI parsing
use clap::Parser;

// Custom REPL module
mod repl;


///////////////////////////////////////////////////////////////////////////////////////
// Parse the main CLI args using the clap crate with the Derive API
#[derive(Parser)]
#[command(name = "Model-executor-runtime-Driver (MER-Driver)")]
#[command(author = "Timotej979 @ GitHub")]
#[command(version = "1.0")]
#[command(about = "Runs the driver for concurent model execution", long_about = None)]
struct Cli {
    #[arg(short, long, env = "ALLOW_MODEL_SERVER_RUNTIME_CHANGES", default_value = "false", help = "Allow runtime changes to the model server DB")]
    allow_model_server_runtime_changes: bool,
}



///////////////////////////////////////////////////////////////////////////////////////
fn main() {

    // Initialize the logger
    log::info!("Initializing the logger...");
    env_logger::init();

    // Parse command line arguments
    log::info!("Parsing CLI args...");
    let args = Cli::parse();

    // Print the parsed arguments
    log::info!("Parsed CLI args:");
    log::info!("    - allow_model_server_runtime_changes: {}", args.allow_model_server_runtime_changes);



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