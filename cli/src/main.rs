extern crate soft;

use soft::client::SoftClient;
use std::io::{BufRead, Read, Write, stdin};
use std::net::TcpStream;

fn main() {
    let stream = TcpStream::connect(("127.0.0.1", soft::DEFAULT_PORT)).unwrap();
    let mut client = SoftClient::new(stream);
    loop {
        let readed = readline();
        let mut splitted = readed.split_whitespace().map(|s| s.to_owned()).collect::<Vec<String>>();
        match splitted[0].as_str() {
            "login" => {
                if splitted.len() == 3 {
                    client.login(&splitted[1], &splitted[2]);
                }
            }
            "get" => {
                if splitted.len() == 2 {
                    let data = client.get(&splitted[1]).unwrap();
                    let string = String::from_utf8(data).unwrap();
                    println!("{}", string);
                }
            }
            "put" => {
                if splitted.len() == 3 {
                    client.put(&splitted[1], &splitted[2]).unwrap();
                }
            }
            "list" => {
                if splitted.len() == 2 {
                    let list = client.list(&splitted[1]).unwrap();
                    for file in list {
                        println!("- {}", file);
                    }
                }
            }
            "exit" => {
                client.exit().unwrap();
                break;
            }
            c => println!("Unknown command {}", c),
        }
    }
}

fn readline() -> String {
    let stdin = stdin();
    let mut lock = stdin.lock();
    let mut buf = String::new();
    lock.read_line(&mut buf).unwrap();
    buf
}