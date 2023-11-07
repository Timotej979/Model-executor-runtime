use std::io;
use std::io::BufRead;

// Logging with env_logger
use env_logger;
use log;

// CLI arg parsing with clap
use clap::Parser;


///////////////////////////////////////////////////////////////////////////////////////
// Parse CLI args using the clap crate with the Derive API
#[derive(Parser)]
#[command(name = "Model-executor-runtime-Driver (MER-Driver)")]
#[command(author = "Timotej979 @ GitHub")]
#[command(version = "1.0")]
#[command(about = "Runs the driver for concurent model execution", long_about = None)]
struct Cli {
    #[arg(short, long, env = "ARG_1", default_value = "default value")]
    arg1: String,
}




fn main() {

    // Initialize the logger
    log::info!("Initializing the logger...");
    env_logger::init();

    // Parse command line arguments
    log::info!("Parsing CLI args...");
    let args = Cli::parse();


    ///////////////////////////////////////////////////////////////////////////////////////
    // Start the CLI loop
    log::info!("Starting the CLI loop...");

    let stdin = io::stdin();
    let mut stdinLine = String::new();
    
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    loop {
        // Read from stdin
        stdin.read_line(&mut stdinLine);

        // Write to stdout
        stdout.write_all(stdinLine.as_bytes()).unwrap();
    }
}