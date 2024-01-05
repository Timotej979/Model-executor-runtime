// src/meal/ssh.rs
use super::{MEALDriver, MEALArgs};
use std::fmt;
use std::collections::HashMap;
use async_trait::async_trait;


// tokio libraries
use tokio::sync::mpsc;
use tokio::task;
use tokio::net::TcpStream;

// SSH library
use makiko;


// Create the SSHDriver struct
pub struct SSHDriver {
    static_fields: HashMap<String, String>,
    model_params: HashMap<String, String>,
    connection_params: HashMap<String, String>,
}

#[async_trait]
impl MEALDriver for SSHDriver {

    ////////////////////////////////////////////////////
    ///// Management of the SSHDriver instance /////
    ////////////////////////////////////////////////////

    fn new(meal_args: MEALArgs) -> Self {
        Self {
            static_fields: meal_args.meal_config[0].clone(),
            model_params: meal_args.meal_config[1].clone(),
            connection_params: meal_args.meal_config[2].clone(),
        }
    }

    //////////////////////////////////////////////////////
    /////// Management of the SSHDriver connection ///////
    //////////////////////////////////////////////////////
    async fn spawn_model(&mut self) -> Result<(mpsc::Sender<String>, mpsc::Receiver<String>, mpsc::Receiver<String>), String> {
        // Get the host
        let host = self.connection_params.get("host").ok_or_else(|| {
            log::error!("Failed to get the host");
            "Failed to get the host".to_string()
        })?;
        // Get the port
        let port = self.connection_params.get("port").ok_or_else(|| {
            log::error!("Failed to get the port");
            "Failed to get the port".to_string()
        })?;
        // Get the username
        let username = self.connection_params.get("user").ok_or_else(|| {
            log::error!("Failed to get the username");
            "Failed to get the username".to_string()
        })?;
        // Get the password
        let password = self.connection_params.get("pass").ok_or_else(|| {
            log::error!("Failed to get the password");
            "Failed to get the password".to_string()
        })?;
        // Get the model path
        let model_path = self.model_params.get("path").ok_or_else(|| {
            log::error!("Failed to get the model path");
            "Failed to get the model path".to_string()
        })?;
        // Get the model command
        let model_command = self.model_params.get("command").ok_or_else(|| {
            log::error!("Failed to get the model command");
            "Failed to get the model command".to_string()
        })?;
        // Log the connection parameters
        log::info!("Connection parameters:\n    - Host: {:#?}\n    - Port: {:#?}\n    - Username: {:#?}\n    - Password: {:#?}", host, port, username, password);
        // Log the model parameters
        log::info!("Model parameters:\n    - Model path: {:#?}\n    - Model command: {:#?}", model_path, model_command);

        /*
        // Open a TCP connection to the host
        let socket = tokio::net::TcpStream::connect((host, port)).await
            .expect("Could not open a TCP socket");

        // Create the client config
        let config = makiko::ClientConfig::default();
        // Open the client
        let (client, mut client_rx, client_fut) = makiko::Client::open(socket, config).ok_or_else(|| {
            log::error!("Failed to open the client");
            "Failed to open the client".to_string()
        })?;

        
        // Spawn a Tokio task that polls the client.
        tokio::task::spawn(async move {
            client_fut.await.ok_or_else(|| {
                log::error!("Error while polling the client");
                "Error while polling the client".to_string()
            });
        });
        

        // Spawn another Tokio task to handle the client events.
        tokio::task::spawn(async move {
            loop {
                // Wait for the next event.
                let event = client_rx.recv().await.ok_or_else(|| {
                    log::error!("Error while receiving the next event");
                    "Error while receiving the next event".to_string()
                });

                // Exit the loop when the client has closed.
                let Some(event) = event else {
                    break
                };

                match event {
                    // Handle the server public key: for now, we just accept all keys, but this makes
                    // us susceptible to man-in-the-middle attacks!
                    makiko::ClientEvent::ServerPubkey(pubkey, accept) => {
                        println!("Server pubkey type {}, fingerprint {}", pubkey.type_str(), pubkey.fingerprint());
                        accept.accept();
                    },

                    // All other events can be safely ignored
                    _ => {},
                }
            }
        });


        // Try to authenticate using a password.
        let auth_res = client.auth_password(username.into(), password.into()).await.ok_or_else(|| {
            log::error!("Error while authenticating");
            "Error while authenticating".to_string()
        });

        // Deal with all possible outcomes of password authentication.
        match auth_res {
            makiko::AuthPasswordResult::Success => {
                log::info!("Successfully authenticated");
            },
            makiko::AuthPasswordResult::ChangePassword(prompt) => {
                log::error!("The server asked us to change our password: {}", prompt);
                return Err("The server asked us to change our password {}".to_string() + prompt);
            },
            makiko::AuthPasswordResult::Failure(failure) => {
                log::error!("Authentication failed: {}", failure);
                return Err("Authentication failed: {}".to_string() + failure);
            },
        }


        // Open a session on the server.
        let channel_config = makiko::ChannelConfig::default();
        let (session, mut session_rx) = client.open_session(channel_config).await.ok_or_else(|| {
            log::error!("Failed to open a session");
            "Failed to open a session".to_string()
        })?;

        // Handle session events asynchronously in a Tokio task.
        let session_event_task = tokio::task::spawn(async move {
            loop {
                // Wait for the next event.
                let event = session_rx.recv().await
                    .expect("Error while receiving session event");

                // Exit the loop when the session has closed.
                let Some(event) = event else {
                    break
                };

                match event {
                    // Handle stdout/stderr output from the process.
                    makiko::SessionEvent::StdoutData(data) => {
                        println!("Process produced stdout: {:?}", data);
                    },
                    makiko::SessionEvent::StderrData(data) => {
                        println!("Process produced stderr: {:?}", data);
                    },

                    // Handle exit of the process.
                    makiko::SessionEvent::ExitStatus(status) => {
                        println!("Process exited with status {}", status);
                    },
                    makiko::SessionEvent::ExitSignal(signal) => {
                        println!("Process exited with signal {:?}: {:?}", signal.signal_name, signal.message);
                    },

                    // Ignore other events
                    _ => {},
                }
            }
        });

        // Execute a command on the session
        session.exec("sed s/blue/green/".as_bytes())
            .expect("Could not execute a command in the session")
            .wait().await
            .expect("Server returned an error when we tried to execute a command in the session");

        // Send some data to the standard input of the process
        session.send_stdin("blueberry jam\n".into()).await.unwrap();
        session.send_stdin("blue jeans\nsky blue".into()).await.unwrap();
        session.send_eof().await.unwrap();

        // Wait for the task that handles session events
        session_event_task.await.unwrap();

        */

        let (stdin_tx, stdin_rx) = mpsc::channel::<String>(1);
        let (stdout_tx, stdout_rx) = mpsc::channel::<String>(1);
        let (stderr_tx, stderr_rx) = mpsc::channel::<String>(1);

        Ok((stdin_tx, stdout_rx, stderr_rx))
    }

}

// Implementation of debug for SSHDriver
impl fmt::Debug for SSHDriver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Print all the fields of SSHDriver
        f.debug_struct("SSHDriver")
            .field("static_fields", &self.static_fields)
            .field("model_params", &self.model_params)
            .field("connection_params", &self.connection_params)
            .finish()
    }
}