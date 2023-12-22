// src/meal/ssh.rs
use super::{MEALDriver, MEALArgs};
use async_trait::async_trait;
use std::collections::HashMap;

// std libraries
use std::io::{self, Read};
use std::net::TcpStream;
use std::time::Duration;

// tokio libraries
use tokio::time::sleep;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// ssh libraries
use ssh2::Session;


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
    async fn spawn_model(&self) -> Result<tokio::process::Child, String> {
        
        // Get the host, port, username and password
        let host = match self.connection_params.get("host") {
            Some(host) => host,
            None => {
                log::error!("Failed to get the host");
                return Err("Failed to get the host".to_string());
            }
        };

        let port = match self.connection_params.get("port") {
            Some(port) => port,
            None => {
                log::error!("Failed to get the port");
                return Err("Failed to get the port".to_string());
            }
        };

        let username = match self.connection_params.get("user") {
            Some(username) => username,
            None => {
                log::error!("Failed to get the username");
                return Err("Failed to get the username".to_string());
            }
        };
        
        let password = match self.connection_params.get("pass") {
            Some(password) => password,
            None => {
                log::error!("Failed to get the password");
                return Err("Failed to get the password".to_string());
            }
        };

        log::info!("Connecting to {}:{} with username {}", host, port, username);


        // Create the TCP stream with error handling
        let tcp_stream = match TcpStream::connect(format!("{}:{}", host, port)).unwrap() {
            Ok(tcp_stream) => tcp_stream,
            Err(e) => {
                log::error!("Failed to connect to {}:{} with username {}", host, port, username);
                return Err("Failed to connect to ".to_string() + host + ":" + port + " with username " + username);
            }
        };

        // Create the SSH session with error handling
        let mut session = match Session::new() {
            Ok(session) => session,
            Err(e) => {
                log::error!("Failed to create the SSH session");
                return Err("Failed to create the SSH session".to_string());
            }
        };

        // Set the TCP stream for the SSH session
        session.set_tcp_stream(tcp_stream);

        // Handshake the SSH session
        match session.handshake(&tcp_stream) {
            Ok(_) => (),
            Err(e) => {
                log::error!("Failed to handshake the SSH session");
                return Err("Failed to handshake the SSH session".to_string());
            }
        };

        // Authenticate the SSH session
        match session.userauth_password(username, password) {
            Ok(_) => (),
            Err(e) => {
                log::error!("Failed to authenticate the SSH session");
                return Err("Failed to authenticate the SSH session".to_string());
            }
        };

        


        

        Ok()
    }

}