use APP_INFO;
use app_dirs::{AppDataType, app_dir};
use error::*;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::Arc;
use super::users::Users;
use types::*;

pub struct SoftConnection<S: Read + Write> {
    root: Option<PathBuf>,
    cwd: String,
    stream: S,
    sender: mpsc::Sender<u8>,
    users: Arc<Users>,
    allow_anonymous: bool,
}

impl<S: Read + Write> SoftConnection<S> {
    pub fn new(stream: S,
               sender: mpsc::Sender<u8>,
               users: Arc<Users>,
               allow_anonymous: bool)
               -> SoftConnection<S> {
        SoftConnection {
            root: None,
            cwd: String::new(),
            stream: stream,
            sender: sender,
            users: users,
            allow_anonymous: allow_anonymous,
        }
    }

    /// TODO Check path
    pub fn run(&mut self) -> Result<()> {
        loop {
            match self.read_command()? {
                Command::Login(u, p) => {
                    if !self.users.is_valid(&u, &p) {
                        self.write_status(Status::WrongLogin)?;
                        continue;
                    }
                    self.write_status(Status::Connected)?;
                    self.root = Some(app_dir(AppDataType::UserData,
                                             &APP_INFO,
                                             format!("users/{}", u).as_str())?);
                    self.cwd = "/".to_string();
                }
                Command::Get(p) => {
                    if self.root.is_none() {
                        self.write_status(Status::NotConnected)?;
                        continue;
                    }
                    self.write_status(Status::Okay)?;
                    self.send_file(&p)?;
                }
                Command::Put(p) => {
                    if self.root.is_none() {
                        self.write_status(Status::NotConnected)?;
                        continue;
                    }
                    self.write_status(Status::Okay)?;
                    let root = self.root.clone().unwrap();
                    let path = root.join(p);
                    let data = self.recv_file()?;
                    let mut file = File::create(&path)?;
                    file.write(data.as_slice())?;
                }
                Command::List(p) => {
                    if self.root.is_none() {
                        self.write_status(Status::NotConnected)?;
                        continue;
                    }
                    self.write_status(Status::Okay)?;
                    let path = self.to_server_path(&p);
                    self.send_list_file(&path)?;
                }
                Command::Cwd => {
                    if self.root.is_none() {
                        self.write_status(Status::NotConnected)?;
                        continue;
                    }
                    self.write_status(Status::Okay)?;
                    self.stream.write(format!("{}\n", self.cwd).as_bytes())?;
                }
                Command::Cd(p) => {
                    if self.root.is_none() {
                        self.write_status(Status::NotConnected)?;
                        continue;
                    }
                    self.cwd = self.to_server_path(&p);
                    self.write_status(Status::Okay)?;
                }
                Command::Exit => {
                    self.write_status(Status::Disconnected)?;
                    break;
                }
            }
        }
        self.sender.send(1).unwrap();
        Ok(())
    }

    /// Receive file from client
    fn recv_file(&mut self) -> Result<Vec<u8>> {
        ::common::recv_file(&mut self.stream)
    }

    /// Send file to client
    fn send_file(&mut self, path: &str) -> Result<()> {
        ::common::send_file(&mut self.stream, path)
    }

    /// Send list of file
    fn send_list_file(&mut self, local_path: &str) -> Result<()> {
        let list = self.list_files(local_path)?;
        ::common::send_list_file(&mut self.stream, list)
    }

    /// Read command sended by client
    fn read_command(&mut self) -> Result<Command> {
        let mut buf = String::new();
        ::common::read_line(&mut self.stream, &mut buf)?;
        Command::try_from(buf)
    }

    /// Write status to client
    fn write_status(&mut self, status: Status) -> Result<()> {
        let status = status as u8;
        self.stream.write(&[status])?;
        Ok(())
    }

    /// Return a valid path from root
    fn to_server_path(&self, path: &str) -> String {
        // TODO return err
        let root = self.root.clone().unwrap();
        let root_str = root.display().to_string();
        let mut path_str = if path.starts_with("/") {
            let p = root.join(path);
            let canonicalized = p.canonicalize().unwrap();
            canonicalized.display().to_string()
        } else {
            let cwd_path = format!("{}/{}", root_str, self.cwd);
            let p = PathBuf::from(cwd_path).join(path);
            let canonicalized = p.canonicalize().unwrap();
            canonicalized.display().to_string()
        };
        if path_str.contains(&root_str) {
            path_str.drain(root_str.len()..).collect()
        } else {
            "/".to_string()
        }
    }

    /// List files from root path
    fn list_files(&self, server_path: &str) -> Result<Vec<String>> {
        let root = self.root.clone().unwrap();
        let root_str = ::common::beautify_path(&format!("{}/{}", root.display(), server_path));
        let mut list = Vec::new();
        let path = root.join(root_str);
        let path_name = ::common::beautify_path(server_path);
        let server_path = ::common::beautify_path(server_path);
        if path.is_dir() {
            for entry in path.read_dir()? {
                let entry = entry?;
                let file_name = entry.file_name();
                let file_name = file_name.to_str().unwrap();
                let mut file_str = format!("{}/{}", server_path, file_name);
                if entry.path().is_dir() {
                    file_str.push('/');
                }
                list.push(file_str);
            }
        } else {
            list.push(path_name);
        }
        Ok(list)
    }
}
