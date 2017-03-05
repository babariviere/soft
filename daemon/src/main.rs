extern crate soft_core;

use soft_core::server::SoftServer;
use std::net::TcpListener;

pub fn main() {
    // TODO cli parsing and configuration
    let mut server = SoftServer::new("soft-daemon", Some(8), true).unwrap();
    server.get_users().add_user("soft", "soft");
    let listener = TcpListener::bind(("127.0.0.1", soft_core::DEFAULT_PORT)).unwrap();
    println!("Listening for client...");
    for stream in listener.incoming() {
        println!("New client connected");
        match stream {
            Ok(stream) => {
                server.new_connection(stream);
            }
            Err(e) => panic!("{}", e),
        }
    }
}
