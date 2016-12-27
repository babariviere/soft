pub mod connection;

use self::connection::SoftConnection;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct SoftServer {
    connection_handlers: Vec<mpsc::Receiver<u8>>,
    users: Arc<HashMap<String, String>>,
    max_threads: usize,
}

impl SoftServer {
    /// Initialize a new server from stream
    pub fn new(max_threads: Option<usize>) -> SoftServer {
        let mut max_threads = max_threads.unwrap_or(8);
        if max_threads < 1 {
            max_threads = 1;
        }
        SoftServer {
            connection_handlers: Vec::new(),
            users: Arc::new(HashMap::new()),
            max_threads: max_threads,
        }
    }

    // TODO
    pub fn load_db(&mut self, path: &str) {
        unimplemented!()
    }

    /// Add a new connection to server
    pub fn new_connection<S: Read + Write + Send + 'static>(&mut self, stream: S) {
        while self.connection_handlers.len() + 1 > self.max_threads {
            self.check_connections();
            thread::sleep(Duration::from_secs(5));
        }
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let mut connection = SoftConnection::new(stream, tx);
            match connection.run() {
                Ok(_) => {
                    // TODO log here
                }
                Err(_) => {
                    // TODO log here
                }
            }
        });
        self.connection_handlers.push(rx);
    }

    /// Check connections and remove those who are stopped
    fn check_connections(&mut self) {
        self.connection_handlers.retain(|c| {
            let recv = c.try_recv();
            recv.is_err()
        });
    }
}

impl Drop for SoftServer {
    fn drop(&mut self) {
        for connection in self.connection_handlers.iter() {
            connection.recv();
        }
    }
}
