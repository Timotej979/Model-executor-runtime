// Std lib imports
use std::io::Write;

// Logging with env_logger
use env_logger;
use log;

// CLI arg parsing with clap
use clap::{Parser, Command, Arg};


///////////////////////////////////////////////////////////////////////////////////////
// Parse the main CLI args using the clap crate with the Derive API
#[derive(Parser)]
#[command(name = "Model-executor-runtime-Driver (MER-Driver)")]
#[command(author = "Timotej979 @ GitHub")]
#[command(version = "1.0")]
#[command(about = "Runs the driver for concurent model execution", long_about = None)]
struct Cli {
    #[arg(short, long, env = "ARG_1", default_value = "default value")]
    arg1: String,
}


///////////////////////////////////////////////////////////////////////////////////////
// Parse REPL command args using the clap crate with the Builder API
struct CliReplManager {
    stdin: std::io::Stdin,
    stdout: std::io::Stdout,
    stderr: std::io::Stderr,
    line: String,
}

impl CliReplManager {
    // Creates a new CliReplManager
    fn new(stdin: std::io::Stdin, stdout: std::io::Stdout, stderr: std::io::Stderr) -> Result<Self, std::io::Error> {
        Ok(Self {
            stdin,
            stdout,
            stderr,
            line: String::new(),
        })
    }

    // Starts the REPL
    fn repl(&mut self) -> Result<(), String> {
        loop {
            // Read a line from stdin and trim it
            self.line = self.read_line()?;
            self.line = self.line.trim().to_string();

            // If the line is empty, continue
            if self.line.is_empty() {
                continue;
            }
    
            // Match the line against the commands
            match self.respond() {
                // If the command is quit, break the loop
                Ok(quit) => {
                    if quit {
                        break;
                    }
                }
                // If the command is not quit, print the output
                Err(err) => {
                    write!(self.stderr, "CLI error: {err}\n").map_err(|err| err.to_string())?;
                    self.stderr.flush().map_err(|err| err.to_string())?;
                }
            }
        }

        // Return Ok
        Ok(())
    }

    // Reads a line from stdin
    fn read_line(&mut self) -> Result<String, String> {
        // Write the prompt
        write!(self.stdout, "$ ").map_err(|e| e.to_string())?;
        self.stdout.flush().map_err(|e| e.to_string())?;
    
        // Read the input
        let mut buffer = String::new();
        self.stdin.read_line(&mut buffer).map_err(|e| e.to_string())?;
    
        // Return the input
        Ok(buffer)
    }

    // Parses the custom CLI arguments
    fn command_parser() -> Command {
        // strip out usage
        const PARSER_TEMPLATE: &str = "\
        {all-args}
        ";
        // strip out name/version
        const APPLET_TEMPLATE: &str = "\
            {about-with-newline}\n\
            {usage-heading}\n    {usage}\n\
            \n\
            {all-args}{after-help}\
        ";

        Command::new("repl")
            .multicall(true)
            .arg_required_else_help(true)
            .subcommand_required(true)
            .subcommand_value_name("APPLET")
            .subcommand_help_heading("APPLETS")
            .help_template(PARSER_TEMPLATE)
            
            // MER-Driver commands
            .subcommand(
                // MER-Driver version
                Command::new("mer-version")
                    .about("Get the version of the MER-Driver")
                    .help_template(APPLET_TEMPLATE),
            )
            .subcommand(
                // MER-Driver ping model
                Command::new("mer-ping")
                    .about("Ping the remote MER-model")
                    .arg(
                        Arg::new("name")
                            //.about("The name of the MER-model")
                            .required(true)
                            .index(1),
                    )
                    .help_template(APPLET_TEMPLATE),
            )
            .subcommand(
                // MER-Driver execute model
                Command::new("mer-execute")
                    .about("Execute the MER-model")
                    .arg(
                        Arg::new("name")
                            //.about("The name of the MER-model")
                            .required(true)
                            .index(1),
                    )
                    .arg(
                        Arg::new("input")
                            //.about("The input of the MER-model")
                            .required(true)
                            .index(2),
                    )
                    .help_template(APPLET_TEMPLATE),
            )
            .subcommand(
                // Mer-Driver toggle logging severity and format
                Command::new("mer-log")
                    .about("Toggle the logging severity")
                    .arg(
                        Arg::new("severity")
                            //.about("The severity of the logging")
                            .required(true)
                            .index(1),
                    )
                    .arg(
                        Arg::new("format")
                            //.about("The format of the logs")
                            .required(true)
                            .index(2),
                    )
                    .help_template(APPLET_TEMPLATE),
            )
            .subcommand(
                // MER-Driver toggle feedback learning for a model
                Command::new("mer-toggle-feedback")
                    .about("Toggle feedback learning for a model")
                    .arg(
                        Arg::new("name")
                            //.about("The name of the MER-model")
                            .required(true)
                            .index(1),
                    )
                    .help_template(APPLET_TEMPLATE),
            )
            .subcommand(
                Command::new("mer-quit")
                    .alias("mer-exit")
                    .about("Quit the MER-repl")
                    .help_template(APPLET_TEMPLATE),
            )
    }

    // Responds to the CLI command
    fn respond(&mut self) -> Result<bool, String> {
        // Split the line into arguments
        let args = shlex::split(&self.line).ok_or("Error: Invalid quoting")?;
    
        // Parse the arguments
        let matches = CliReplManager::command_parser()
            .try_get_matches_from(args)
            .map_err(|e| e.to_string())?;
    
        // Match the subcommand
        match matches.subcommand() {
            Some(("ping", _matches)) => {
                write!(self.stdout, "Pong\n").map_err(|e| e.to_string())?;
                self.stdout.flush().map_err(|e| e.to_string())?;
            }
            Some(("quit", _matches)) => {
                write!(self.stdout, "Exiting ...\n").map_err(|e| e.to_string())?;
                self.stdout.flush().map_err(|e| e.to_string())?;
                // Return true
                return Ok(true);
            }
            Some((name, _matches)) => unimplemented!("{name}\n"),
            None => unreachable!("Error: Subcommand required\n"),
        }
    
        // Return false
        Ok(false)
    }

}







///////////////////////////////////////////////////////////////////////////////////////
fn main() {

    // Initialize the logger
    log::info!("Initializing the logger...");
    env_logger::init();

    // Parse command line arguments
    log::info!("Parsing CLI args...");
    let args = Cli::parse();


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
    let mut crm_instance = CliReplManager::new(stdin, stdout, stderr).expect("Failed to initialize the CliReplManager");

    // Start the REPL
    let _ = crm_instance.repl();

}