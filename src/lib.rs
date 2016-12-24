#[macro_use]
extern crate error_chain;

pub mod client;
pub mod error;
pub mod server;
pub mod types;

pub const DEFAULT_PORT: u16 = 9045;

#[cfg(test)]
mod tests {
    use client::SoftClient;
    use server::SoftServer;
    use std::fs;
    use std::io::Write;
    use std::net;
    use std::thread;
    use types::*;

    #[test]
    fn file_stream() {
        let server_stream = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(".file_stream")
            .unwrap();
        let client_stream =
            fs::OpenOptions::new().read(true).write(true).open(".file_stream").unwrap();
        let mut server = SoftServer::new(server_stream);
        let mut client = SoftClient::new(client_stream);
        client.write_command(Command::Exit).unwrap();
        assert_eq!(server.read_command().unwrap(), Command::Exit);
        fs::remove_file(".file_stream").unwrap();
    }

    #[test]
    fn tcp_stream() {
        let server_stream = net::TcpListener::bind(("0.0.0.0", super::DEFAULT_PORT)).unwrap();
        let addr = server_stream.local_addr().unwrap();
        let server_thread = thread::spawn(move || {
            let (client, _) = server_stream.accept().unwrap();
            let mut server = SoftServer::new(client);
            assert_eq!(server.read_command().unwrap(), Command::Exit);
        });
        let client_stream = net::TcpStream::connect(addr).unwrap();
        let mut client = SoftClient::new(client_stream);
        client.write_command(Command::Exit).unwrap();
        server_thread.join().unwrap();
    }

    #[test]
    fn file_transfert() {
        let server_stream = net::TcpListener::bind(("0.0.0.0", super::DEFAULT_PORT + 1)).unwrap();
        let addr = server_stream.local_addr().unwrap();
        let server_thread = thread::spawn(move || {
            let (client, _) = server_stream.accept().unwrap();
            let mut server = SoftServer::new(client);
            server.read_command().unwrap();
            server.send_file("Cargo.toml").unwrap();
        });
        let client_stream = net::TcpStream::connect(addr).unwrap();
        let mut client = SoftClient::new(client_stream);
        let _data = client.get("Cargo.toml").unwrap();
        server_thread.join().unwrap();
    }

    #[test]
    fn file_transfert_file_stream() {
        let server_stream = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(".file_streamt")
            .unwrap();
        let client_stream =
            fs::OpenOptions::new().read(true).write(true).open(".file_streamt").unwrap();
        let mut server = SoftServer::new(server_stream);
        let mut client = SoftClient::new(client_stream);
        let server_thread = thread::spawn(move || {
            server.read_command().unwrap();
            server.send_file("Cargo.toml").unwrap();
        });
        let _data = client.get("Cargo.toml").unwrap();
        server_thread.join().unwrap();
        fs::remove_file(".file_streamt").unwrap();
    }

    #[test]
    #[should_panic]
    fn file_transfert_fail() {
        let server_stream = net::TcpListener::bind(("0.0.0.0", super::DEFAULT_PORT + 2)).unwrap();
        let addr = server_stream.local_addr().unwrap();
        let server_thread = thread::spawn(move || {
            let (client, _) = server_stream.accept().unwrap();
            let mut server = SoftServer::new(client);
            server.send_file("Cargo.toml").unwrap();
        });
        let client_stream = net::TcpStream::connect(addr).unwrap();
        let mut client = SoftClient::new(client_stream);
        let _data = client.get("Cargo.toml").unwrap();
        server_thread.join().unwrap();
    }
}
