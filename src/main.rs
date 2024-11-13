
pub mod http;
pub mod server;
pub mod routes;
pub mod request;
pub mod response;

use std::thread;
use std::net::TcpListener;

use crate::server::Server;

fn main() {
    let server: TcpListener = Server::new("127.0.0.1:80".to_string())
        .start().unwrap();
    println!("Listening on port 127.0.0.1:80");

    for stream in server.incoming() {
        match stream {
            Ok(stream) => { thread::spawn(move || Server::handle_request(stream)).join().unwrap() }
            Err(e) => { println!("Error: {}", e) }
        }
    }
}