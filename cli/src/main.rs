extern crate soft_core;

use soft_core::client::SoftClient;
use std::io::{BufRead, stdin};
use std::net::TcpStream;

pub fn main() {
    let stream = TcpStream::connect(("127.0.0.1", soft_core::DEFAULT_PORT)).unwrap();
    let mut client = SoftClient::new(stream);
    loop {
        let readed = readline();
        let splitted = readed.split_whitespace().map(|s| s.to_owned()).collect::<Vec<String>>();
        match splitted[0].as_str() {
            "login" => {
                if splitted.len() == 3 {
                    client.login(&splitted[1], &splitted[2]).unwrap();
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
            "listr" => {
                if splitted.len() == 2 {
                    let list = client.list_recursive(&splitted[1]).unwrap();
                    for file in list {
                        println!(" - {}", file);
                    }
                }
            }
            "cwd" => {
                println!("{}", client.cwd().unwrap());
            }
            "mkdir" => {
                if splitted.len() == 2 {
                    client.mkdir(&splitted[1]).unwrap();
                }
            }
            "cd" => {
                if splitted.len() == 2 {
                    client.cd(&splitted[1]).unwrap();
                }
            }
            "rm" => {
                if splitted.len() == 2 {
                    client.rm(&splitted[1]).unwrap();
                }
            }
            "rmdir" => {
                if splitted.len() == 2 {
                    client.rmdir(&splitted[1], true).unwrap();
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
