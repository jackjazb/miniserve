mod http;
mod renderer;
mod router;
mod server;
use clap::Parser;
use std::{
    io::Write,
    net::TcpListener,
    process::exit,
    time::{Duration, SystemTime},
};

use server::Server;

const DEFAULT_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: u32 = 8080;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Config {
    /// The directory to serve markdown from
    directory: String,

    /// Specify an address to bind to
    #[arg(short, long, default_value_t = String::from(DEFAULT_ADDRESS))]
    address: String,
    /// Specify a port to bind to
    #[arg(short, long, default_value_t = DEFAULT_PORT)]
    port: u32,
}

fn main() {
    let init_time = SystemTime::now();
    let config = Config::parse();

    // Create a new server instance from a directory
    if let Some(server_instance) = Server::new(&config.directory) {
        let mut address = String::from(&config.address);
        address.push(':');
        address.push_str(&config.port.to_string());

        // Panic if the a TCP listener cannot be bound
        let listener = TcpListener::bind(address).expect("Failed to start server.");

        let startup_time = init_time.elapsed();

        println!(
            "Server started in {:?} // Listening on port {:?}",
            startup_time.unwrap_or(Duration::ZERO),
            config.port
        );

        for stream in listener.incoming() {
            if let Ok(mut s) = stream {
                match server_instance.handle_connection(&s) {
                    Ok(res) => {
                        let bytes: Vec<u8> = res.into();
                        let write_result = s.write_all(&bytes);
                        if write_result.is_err() {
                            println!("Failed to write response: {:?}", write_result);
                        }
                    }
                    Err(err) => println!("Last connection failed: {:?}", err),
                }
            }
        }
    } else {
        println!("Failed to load mardown from path {}", config.directory);
        exit(1);
    }
}
