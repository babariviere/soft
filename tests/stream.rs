extern crate soft;

use soft::client::SoftClient;
use soft::server::SoftServer;
use soft::types::*;
use std::fs;
use std::net;
use std::thread;

const FILE_NAME: &'static str = "Cargo.toml";
const FILE_DATA: &'static str = include_str!("../Cargo.toml");

#[test]
fn file_stream() {
    let server_stream = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(".file_stream")
        .unwrap();
    let client_stream = fs::OpenOptions::new().read(true).write(true).open(".file_stream").unwrap();
    let mut server = SoftServer::new(server_stream);
    let mut client = SoftClient::new(client_stream);
    client.write_command(Command::Exit).unwrap();
    assert_eq!(server.read_command().unwrap(), Command::Exit);
    fs::remove_file(".file_stream").unwrap();
}

#[test]
fn tcp_stream() {
    let server_stream = net::TcpListener::bind(("0.0.0.0", soft::DEFAULT_PORT)).unwrap();
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
    let server_stream = net::TcpListener::bind(("0.0.0.0", soft::DEFAULT_PORT + 1)).unwrap();
    let addr = server_stream.local_addr().unwrap();
    let server_thread = thread::spawn(move || {
        let (client, _) = server_stream.accept().unwrap();
        let mut server = SoftServer::new(client);
        server.read_command().unwrap();
        server.send_file(FILE_NAME).unwrap();
        server.read_command().unwrap();
        let data = server.recv_file().unwrap();
        assert_eq!(data, FILE_DATA.as_bytes());
    });
    let client_stream = net::TcpStream::connect(addr).unwrap();
    let mut client = SoftClient::new(client_stream);
    let data = client.get(FILE_NAME).unwrap();
    assert_eq!(data, FILE_DATA.as_bytes());
    client.put(FILE_NAME, "Cargo.t").unwrap();
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
    client.write_command(Command::Get(FILE_NAME.into())).unwrap();
    server.read_command().unwrap();
    server.send_file(FILE_NAME).unwrap();
    let data = client.recv_file().unwrap();
    fs::remove_file(".file_streamt").unwrap();
    assert_eq!(data, FILE_DATA.as_bytes());
}

#[test]
#[should_panic]
fn file_transfert_fail() {
    let server_stream = net::TcpListener::bind(("0.0.0.0", soft::DEFAULT_PORT + 2)).unwrap();
    let addr = server_stream.local_addr().unwrap();
    let server_thread = thread::spawn(move || {
        let (client, _) = server_stream.accept().unwrap();
        let mut server = SoftServer::new(client);
        server.read_command().unwrap();
        let data = server.recv_file().unwrap();
        assert_eq!(data, FILE_DATA.as_bytes());
    });
    let client_stream = net::TcpStream::connect(addr).unwrap();
    let mut client = SoftClient::new(client_stream);
    client.send_file(FILE_NAME).unwrap();
    server_thread.join().unwrap();
}
