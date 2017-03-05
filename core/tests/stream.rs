extern crate soft_core;

use soft_core::client::SoftClient;
use soft_core::server::SoftServer;
use soft_core::types::*;
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
    let mut server = SoftServer::new("test_file_stream", None, true).unwrap();
    let mut client = SoftClient::new(client_stream);
    server.new_connection(server_stream);
    client.write_command(Command::Exit).unwrap();
    fs::remove_file(".file_stream").unwrap();
}

#[test]
fn tcp_stream() {
    let mut server = SoftServer::new("test_tcp_stream", None, true).unwrap();
    let server_stream = net::TcpListener::bind(("0.0.0.0", soft_core::DEFAULT_PORT)).unwrap();
    let addr = server_stream.local_addr().unwrap();
    let server_thread = thread::spawn(move || {
        let (client, _) = server_stream.accept().unwrap();
        server.new_connection(client);
    });
    let client_stream = net::TcpStream::connect(addr).unwrap();
    let mut client = SoftClient::new(client_stream);
    client.presence().unwrap();
    client.write_command(Command::Exit).unwrap();
    server_thread.join().unwrap();
}

#[test]
fn file_transfert() {
    let mut server = SoftServer::new("test_file_transfert", None, true).unwrap();
    server.get_users().add_user("test", "test");
    let server_stream = net::TcpListener::bind(("0.0.0.0", soft_core::DEFAULT_PORT + 1)).unwrap();
    let addr = server_stream.local_addr().unwrap();
    let server_thread = thread::spawn(move || {
        let (client, _) = server_stream.accept().unwrap();
        server.new_connection(client);
    });
    let client_stream = net::TcpStream::connect(addr).unwrap();
    let mut client = SoftClient::new(client_stream);
    client.presence().unwrap();
    client.login("test", "test").unwrap();
    client.put(FILE_NAME, "Cargo.toml").unwrap();
    let data = client.get(FILE_NAME).unwrap();
    assert_eq!(data, FILE_DATA.as_bytes());
    client.presence().unwrap();
    client.exit().unwrap();
    server_thread.join().unwrap();
}
// #[test]
// fn login() {
//     let server_stream = net::TcpListener::bind(("0.0.0.0", soft_core::DEFAULT_PORT + 2)).unwrap();
//     let addr = server_stream.local_addr().unwrap();
//     let server_thread = thread::spawn(move || {
//         let (client, _) = server_stream.accept().unwrap();
//         let mut server = SoftServer::new(client);
//         let command = server.read_command().unwrap();
//         let (user, pass) = command.unwrap_login();
//         if user == "test" && pass == "test_pass" {
//             server.write_status(Status::Connected).unwrap();
//         } else {
//             server.write_status(Status::WrongLogin).unwrap();
//         }
//     });
//     let client_stream = net::TcpStream::connect(addr).unwrap();
//     let mut client = SoftClient::new(client_stream);
//     client.login("test", "test_pass").unwrap();
//     server_thread.join().unwrap();
// }

#[test]
fn list_files() {
    let mut server = SoftServer::new("test_list_files", None, true).unwrap();
    server.get_users().add_user("test", "test");
    let server_stream = net::TcpListener::bind(("0.0.0.0", soft_core::DEFAULT_PORT + 3)).unwrap();
    let addr = server_stream.local_addr().unwrap();
    let server_thread = thread::spawn(move || {
        let (client, _) = server_stream.accept().unwrap();
        server.new_connection(client);
    });
    let client_stream = net::TcpStream::connect(addr).unwrap();
    let mut client = SoftClient::new(client_stream);
    client.login("test", "test").unwrap();
    let _list = client.list(".//////./////").unwrap();
    client.exit().unwrap();
    server_thread.join().unwrap();
}

#[test]
fn drop_exit() {
    let mut server = SoftServer::new("test_drop_exit", None, true).unwrap();
    let server_stream = net::TcpListener::bind(("0.0.0.0", soft_core::DEFAULT_PORT + 4)).unwrap();
    let addr = server_stream.local_addr().unwrap();
    let server_thread = thread::spawn(move || {
        let (client, _) = server_stream.accept().unwrap();
        server.new_connection(client);
    });
    let client_stream = net::TcpStream::connect(addr).unwrap();
    {
        let mut _client = SoftClient::new(client_stream);
    }
    server_thread.join().unwrap();
}
