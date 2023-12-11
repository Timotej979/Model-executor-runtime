// /src/repl/mod.rs
// Std lib imports
use std::io::Write;

// CLI arg parsing with clap
use clap::{Command, Arg};


// Parse REPL command args using the clap crate with the Builder API
pub struct CliReplManager {
    stdin: std::io::Stdin,
    stdout: std::io::Stdout,
    stderr: std::io::Stderr,
    line: String,
    allow_model_server_runtime_changes: bool,
}

impl CliReplManager {
    // Creates a new CliReplManager
    pub fn new(stdin: std::io::Stdin, stdout: std::io::Stdout, stderr: std::io::Stderr, allow_model_server_runtime_changes: bool) -> Result<Self, std::io::Error> {
        Ok(Self {
            stdin,
            stdout,
            stderr,
            line: String::new(),
            allow_model_server_runtime_changes,
        })
    }

    // Starts the REPL
    pub fn repl(&mut self) -> Result<(), String> {
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
                    write!(std::io::stdout(), "{err}").map_err(|e| e.to_string())?;
                    std::io::stdout().flush().map_err(|e| e.to_string())?;
                }
            }
        }

        Ok(())
    }

    // Reads a line from stdin
    fn read_line(&mut self) -> Result<String, String> {
        // Write the prompt
        write!(self.stdout, "Model-Executor-Runtime-CLI $ ").map_err(|e| e.to_string())?;
        self.stdout.flush().map_err(|e| e.to_string())?;
    
        // Read the input
        let mut buffer = String::new();
        self.stdin.read_line(&mut buffer).map_err(|e| e.to_string())?;
    
        Ok(buffer)
    }

    // Parses the custom CLI arguments
    fn command_parser(&self) -> Command {
        
        // strip out usage
        const PARSER_TEMPLATE: &str = r"
         __  _______  ____  ________       _______  __ ______________  ____________  ____ 
        /  |/  / __ \/ __ \/ ____/ /      / ____/ |/ // ____/ ____/ / / /_  __/ __ \/ __ \
       / /|_/ / / / / / / / __/ / /      / __/  |   // __/ / /   / / / / / / / / / / /_/ /
      / /  / / /_/ / /_/ / /___/ /___   / /___ /   |/ /___/ /___/ /_/ / / / / /_/ / _, _/ 
     /_/  /_/\____/_____/_____/_____/  /_____//_/|_/_____/\____/\____/ /_/  \____/_/ |_|  
                      ____  __  ___   ______________  _________                             
                     / __ \/ / / / | / /_  __/  _/  |/  / ____/                             
                    / /_/ / / / /  |/ / / /  / // /|_/ / __/                                
                   / _, _/ /_/ / /|  / / / _/ // /  / / /___                                
                  /_/ |_|\____/_/ |_/ /_/ /___/_/  /_/_____/                                
                                                                                          
    {all-args}";

        // strip out name/version
        const APPLET_TEMPLATE: &str = "\
            - {about-with-newline}\n\
            - {usage-heading}\n    - {usage}\n\
            \n\
            {all-args}{after-help}\
        ";

        let mut app = Command::new("driver-repl")
            .multicall(true)
            .arg_required_else_help(true)
            .subcommand_required(true)
            .subcommand_value_name("APPLET")
            .help_template(PARSER_TEMPLATE)
            
            // MER-Driver commands
            .subcommand(
                // driver-version
                Command::new("version")
                    .about("Get the version of the MER-river")
                    .help_template(APPLET_TEMPLATE),
            )
            .subcommand(
                // list models
                Command::new("model-list")
                    .alias("list-models")
                    .alias("model-ls")
                    .alias("ls-models")
                    .alias("ls")
                    .about("List all available models")
                    .help_template(APPLET_TEMPLATE),
            )
            .subcommand(
                // model info
                Command::new("model-info")
                    .about("Get info about a model")
                    .arg(
                        Arg::new("name")
                            .help("The name of the model to get info about")
                            .required(true)
                            .index(1),
                    )
                    .help_template(APPLET_TEMPLATE), 
            )
            .subcommand(
                // MER-Driver ping model
                Command::new("model-ping")
                    .alias("ping-model")
                    .alias("ping")
                    .about("Ping the remote model")
                    .arg(
                        Arg::new("name")
                            .help("The name of the model to check if it exists")
                            .required(true)
                            .index(1),
                    )
                    .help_template(APPLET_TEMPLATE),
            )
            .subcommand(
                // MER-Driver execute model
                Command::new("model-execute")
                    .alias("execute-model")
                    .alias("execute")
                    .about("Execute the model")
                    .arg(
                        Arg::new("name")
                            .help("The name of the MER-model")
                            .required(true)
                            .index(1),
                    )
                    .arg(
                        Arg::new("input")
                            .help("The input of the MER-model")
                            .required(true)
                            .index(2),
                    )
                    .help_template(APPLET_TEMPLATE),
            )
            .subcommand(
                // Driver toggle feedback learning for a model
                Command::new("model-continuous-feedback")
                    .alias("continuous-feedback")
                    .alias("feedback")
                    .about("Toggle continuous feedback learning for a model")
                    .arg(
                        Arg::new("name")
                            .help("The name of the model")
                            .required(true)
                            .index(1),
                    )
                    .arg(
                        Arg::new("state")
                            .help("Toggle continuous feedback learning for a model")
                            .required(true)
                            .index(1),
                    )
                    .help_template(APPLET_TEMPLATE),
            )
            .subcommand(
                Command::new("exit")
                    .alias("quit")
                    .about("Quit the MER-repl")
                    .help_template(APPLET_TEMPLATE),
            );


            // Optional control commands for editing the existing models
            if self.allow_model_server_runtime_changes {
                app = app.subcommand(
                    // Modify model entries
                    Command::new("model-modify")
                        .alias("modify")
                        .about("Modify an existing model entry")
                        .help_template(APPLET_TEMPLATE)
                );
                
                app = app.subcommand(
                    Command::new("model-create")
                        .alias("create")
                        .about("Create new model entry")
                        .help_template(APPLET_TEMPLATE),
                );

                app = app.subcommand(
                    Command::new("model-delete")
                        .alias("delete")
                        .about("Delete an existing model entry")
                        .help_template(APPLET_TEMPLATE)
                );
            }

        return app;
    }

    // Responds to the CLI command
    fn respond(&mut self) -> Result<bool, String> {
        // Split the line into arguments
        let args = shlex::split(&self.line).ok_or("Error: Invalid quoting")?;
    
        // Parse the arguments
        let matches = CliReplManager::command_parser(&self)
            .try_get_matches_from(args)
            .map_err(|e| e.to_string())?;


        // Match the subcommand
        match matches.subcommand() {
            Some(("version", _matches)) => {
                write!(self.stdout, "MER-Driver version: 0.1.0\n").map_err(|e| e.to_string())?;
                self.stdout.flush().map_err(|e| e.to_string())?;
            }

            Some(("model-ping", _matches)) => {
                if let Some(name) = _matches.get_one::<String>("name") {
                    write!(self.stdout, "Checking if model {} is available...\n", name).map_err(|e| e.to_string())?;
                    self.stdout.flush().map_err(|e| e.to_string())?;
                } else {
                    write!(self.stdout, "Error: Name argument is missing\n").map_err(|e| e.to_string())?;
                    self.stdout.flush().map_err(|e| e.to_string())?;
                }
            }

            Some(("model-execute", _matches)) => {
                if let (Some(name), Some(input)) = (_matches.get_one::<String>("name"), _matches.get_one::<String>("input")) {
                    write!(self.stdout, "Executing model {} with input {}...\n", name, input).map_err(|e| e.to_string())?;
                    self.stdout.flush().map_err(|e| e.to_string())?;
                } else {
                    write!(self.stdout, "Error: Name or input argument is missing\n").map_err(|e| e.to_string())?;
                    self.stdout.flush().map_err(|e| e.to_string())?;
                }
            }

            Some(("model-toggle-feedback", _matches)) => {
                if let Some(name) = _matches.get_one::<String>("name") {
                    write!(self.stdout, "Toggling feedback learning for model {}...\n", name).map_err(|e| e.to_string())?;
                    self.stdout.flush().map_err(|e| e.to_string())?;
                } else {
                    write!(self.stdout, "Error: Name argument is missing\n").map_err(|e| e.to_string())?;
                    self.stdout.flush().map_err(|e| e.to_string())?;
                }
            }

            Some(("exit", _matches)) => {
                write!(self.stdout, "Exiting Model-Executor Runtime-CLI ...\n").map_err(|e| e.to_string())?;
                self.stdout.flush().map_err(|e| e.to_string())?;
                // Return true
                return Ok(true);
            }

            Some((name, _matches)) => unimplemented!("{name}\n"),
            None => unreachable!("Error: Subcommand required\n"),
        }

        Ok(false)
    }

}