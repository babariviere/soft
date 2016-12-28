use error::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

pub struct Users(Arc<Mutex<HashMap<String, String>>>, PathBuf);

impl Users {
    /// Load a database or create a new one if path doesn't exists
    pub fn load<P: AsRef<Path>>(dir: P) -> Result<Users> {
        let path = dir.as_ref().join("users.db");
        if !path.exists() {
            return Ok(Users(Arc::new(Mutex::new(HashMap::new())), path.to_path_buf()));
        }
        let mut file = File::open(&path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let mut db = HashMap::new();
        for line in buf.split('\n') {
            if line.is_empty() {
                continue;
            }
            let line = line.trim();
            let words = line.split_whitespace().map(|x| x.to_owned()).collect::<Vec<String>>();
            if words.len() > 2 {
                bail!(ErrorKind::InvalidUserDB);
            }
            db.insert(words[0].clone(), words[1].clone());
        }

        Ok(Users(Arc::new(Mutex::new(db)), path.to_path_buf()))
    }

    /// Check if the user provided is present and if his password is valid.
    pub fn is_valid(&self, user: &str, pass: &str) -> bool {
        let lock = self.0.lock().unwrap();
        match lock.get(user) {
            Some(p) => p == pass,
            None => false,
        }
    }

    /// Add a new user to database
    pub fn add_user(&self, user: &str, pass: &str) {
        let mut lock = self.0.lock().unwrap();
        if lock.get(user).is_some() {
            return;
        }
        lock.insert(user.to_owned(), pass.to_owned());
    }
}

impl Drop for Users {
    fn drop(&mut self) {
        let lock = self.0.lock().unwrap();
        let mut file = File::create(&self.1).unwrap();
        for (key, value) in lock.iter() {
            let s = format!("{} {}\n", key, value);
            let _ = file.write_all(s.as_bytes());
        }
    }
}
