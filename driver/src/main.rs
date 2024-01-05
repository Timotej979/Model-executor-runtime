// src/main.rs
// Standard liraries
use std::collections::HashMap;

// Logging with env_logger
use env_logger;
use log;

// CLI parsing
use clap::Parser;

// Custom modules
mod dal;
mod meal;
mod repl;


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
    log::info!("    - allow_model_server_runtime_changes: {:#?}", args.allow_model_server_runtime_changes);


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
            log::error!("Failed to create the DAL instance: {:#?}", error);
            std::process::exit(1);
        }
    };

    // Connect to the DAL
    let _ = match dal_instance.connect().await {
        Ok(_) => (),
        Err(error) => {
            log::error!("Failed to connect to the DAL: {:#?}", error);
            std::process::exit(1);
        }
    };

    // Get the available models
    let available_models = match dal_instance.get_available_models().await {
        Ok(models) => models,
        Err(error) => {
            log::error!("Failed to get available models: {:#?}", error);
            std::process::exit(1);
        }
    };

    // Create the MEAL instances for every available model
    let mut meal_instances: HashMap<String, Vec<meal::MEAL>> = HashMap::new();
    for model in available_models {
        // Get the model name
        let model_name = model[0].get("name").cloned().unwrap_or_default();
        // Get the model connectionType
        let connection_type = match model[0].get("connType") {
            Some(conn_type) => conn_type,
            None => {
                log::error!("Failed to get the connection type for the model: {:#?}", model[0].get("name"));
                std::process::exit(1);
            }
        };
        // Print the model name
        log::info!("Creating the MEAL instance for the model: {:#?} with connection type: {:#?}", model_name, connection_type);
        // Create the MEAL instance
        if connection_type == "ssh" || connection_type == "local" {
            // Create the MEALArgs instance
            let meal_args = meal::MEALArgs {
                meal_config: model.clone(),
            };
            // Create the MEAL instance
            let meal = match meal::MEAL::create(&connection_type, meal_args) {
                Ok(instance) => instance,
                Err(error) => {
                    log::error!("Failed to create the MEAL instance: {:#?}", error);
                    std::process::exit(1);
                }
            };
            // Add the MEAL instance to the vector of MEAL instances for the same model, defined by the model name
            match meal_instances.get_mut(&model_name) {
                Some(meal_vec) => meal_vec.push(meal),
                None => {
                    meal_instances.insert(model_name, vec![meal]);
                }
            }
        } else {
            log::error!("Unsupported connection type: {:#?}", connection_type);
        }
    }

    // Print the MEAL instances
    log::info!("MEAL instances: {:#?}", meal_instances);

    // Select one local MEAL instance via the driver name
    let local_meal_instance = match meal_instances.get_mut("DialogGPT-small") {
        Some(meal_vec) => meal_vec,
        None => {
            log::error!("Failed to get the local MEAL instance");
            std::process::exit(1);
        }
    };

    // Spawn the local MEAL instance
    let (mut local_meal_sender, mut local_meal_receiver, mut local_meal_error_receiver) = match local_meal_instance[0].spawn_model().await {
        Ok((sender, receiver, error_receiver)) => (sender, receiver, error_receiver),
        Err(error) => {
            log::error!("Failed to spawn the local MEAL instance: {:#?}", error);
            std::process::exit(1);
        }
    };

    // Send the message to the local MEAL instance
    let _ = local_meal_sender.send("Hello from the driver!".to_string()).await;
    log::info!("Sent message to the local MEAL instance");

    // Receive the message from the local MEAL instance
    let message = match local_meal_receiver.recv().await {
        Some(message) => message,
        None => {
            log::error!("Failed to receive the message from the local MEAL instance");
            std::process::exit(1);
        }
    };
    log::info!("Received message from the local MEAL instance: {:#?}", message);

    // Receive the error from the local MEAL instance
    let error = match local_meal_error_receiver.recv().await {
        Some(error) => error,
        None => {
            log::error!("Failed to receive the error from the local MEAL instance");
            std::process::exit(1);
        }
    };
    log::info!("Received error from the local MEAL instance: {:#?}", error);
    
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