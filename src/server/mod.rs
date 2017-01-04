mod connection;
pub mod users;

use APP_INFO;
use app_dirs::{AppDataType, app_dir};
use error::*;
use self::connection::SoftConnection;
use self::users::Users;
use std::io::{Read, Write};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct SoftServer {
    connection_handlers: Vec<mpsc::Receiver<u8>>,
    users: Arc<Users>,
    max_threads: usize,
    allow_anonymous: bool,
}

impl SoftServer {
    /// Initialize a new server from stream
    pub fn new(name: &str,
               max_threads: Option<usize>,
               allow_anonymous: bool)
               -> Result<SoftServer> {
        let mut max_threads = max_threads.unwrap_or(8);
        if max_threads < 1 {
            max_threads = 1;
        }
        let path = app_dir(AppDataType::UserData,
                           &APP_INFO,
                           format!("db/{}", name).as_str())?;
        Ok(SoftServer {
            connection_handlers: Vec::new(),
            users: Arc::new(Users::load(path)?),
            max_threads: max_threads,
            allow_anonymous: allow_anonymous,
        })
    }

    /// Add a new connection to server
    pub fn new_connection<S: Read + Write + Send + 'static>(&mut self, stream: S) {
        while self.connection_handlers.len() + 1 > self.max_threads {
            self.check_connections();
            thread::sleep(Duration::from_secs(5));
        }
        let (tx, rx) = mpsc::channel();
        let users = self.users.clone();
        let allow_anonymous = self.allow_anonymous.clone();
        thread::spawn(move || {
            let mut connection = SoftConnection::new(stream, tx, users, allow_anonymous);
            // TODO error handling
            connection.run().unwrap();
        });
        self.connection_handlers.push(rx);
    }

    pub fn get_users(&self) -> Arc<Users> {
        self.users.clone()
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
        for connection in &self.connection_handlers {
            let _ = connection.recv();
        }
    }
}
