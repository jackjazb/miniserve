mod http;
mod renderer;
mod router;
mod server;
mod site;
use std::{
    net::TcpListener,
    time::{Duration, SystemTime},
};

const PORT: u32 = 8080;

fn main() {
    let init_time = SystemTime::now();
    if let Ok(base_routes) = site::load_routes_from_disk("./site") {
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
                let last_conn_result = server::handle_connection(s, &base_routes);
                if last_conn_result.is_err() {
                    println!("Last connection failed: {:? }", last_conn_result);
                }
            }
        }
    }
}
