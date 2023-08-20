mod http;
mod renderer;
mod router;
mod server;
use std::{
    net::TcpListener,
    time::{Duration, SystemTime},
};

use server::Server;

const PORT: u32 = 8080;

fn main() {
    let init_time = SystemTime::now();
    if let Some(server_instance) = Server::new("./site") {
        let mut address = String::from("127.0.0.1");
        address.push(':');
        address.push_str(&PORT.to_string());

        let listener = TcpListener::bind(&address).expect("Failed to start server.");

        let startup_time = init_time.elapsed();

        println!(
            "Server started in {:?} // Listening on port {:?}",
            startup_time.unwrap_or(Duration::ZERO),
            PORT
        );

        for stream in listener.incoming() {
            if let Ok(s) = stream {
                let last_conn_result = server_instance.handle_connection(s);
                if last_conn_result.is_err() {
                    println!("Last connection failed: {:? }", last_conn_result);
                }
            }
        }
    }
}
