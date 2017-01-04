use APP_INFO;
use app_dirs::{AppDataType, app_dir};
use error::*;
use std::fs::{self, File};
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

    /// Run handler for this connection
    pub fn run(&mut self) -> Result<()> {
        loop {
            let command = self.read_command()?;
            match command {
                Command::Login(_, _) |
                Command::Presence |
                Command::Exit => {}
                _ => {
                    if self.root.is_none() {
                        self.write_status(Status::NotConnected)?;
                        continue;
                    }
                }
            }
            match command {
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
                    self.write_status(Status::Okay)?;
                    self.send_file(&p)?;
                }
                Command::Put(p) => {
                    // FIXME new file
                    self.write_status(Status::Okay)?;
                    let path_str = format!("{}/{}",
                                           self.root.clone().unwrap().display(),
                                           self.to_server_path(&p));
                    let path = PathBuf::from(path_str);
                    let data = self.recv_file()?;
                    let mut file = File::create(&path)?;
                    file.write(data.as_slice())?;
                }
                Command::List(p) => {
                    self.write_status(Status::Okay)?;
                    let path = self.to_server_path(&p);
                    self.send_list_file(&path)?;
                }
                Command::Cwd => {
                    self.write_status(Status::Okay)?;
                    self.stream.write(format!("{}\n", self.cwd).as_bytes())?;
                }
                Command::Cd(p) => {
                    self.cwd = self.to_server_path(&p);
                    self.write_status(Status::Okay)?;
                }
                Command::Mkdir(p) => {
                    let path = format!("{}/{}/{}",
                                       self.root.clone().unwrap().display(),
                                       self.cwd,
                                       p);
                    fs::create_dir_all(path)?;
                    self.write_status(Status::Okay)?;
                }
                Command::Rm(p) => {
                    let path_str = self.to_root_path(&p);
                    let path = PathBuf::from(path_str);
                    if !path.exists() {
                        self.write_status(Status::PathUnknown)?;
                        continue;
                    }
                    if path.is_file() {
                        fs::remove_file(path)?;
                        self.write_status(Status::Okay)?;
                    } else {
                        self.write_status(Status::NotFile)?;
                    }
                }
                Command::Rmdir(p, recursive) => {
                    let path_str = self.to_root_path(&p);
                    let path = PathBuf::from(path_str);
                    if !path.exists() {
                        self.write_status(Status::PathUnknown)?;
                        continue;
                    }
                    if path.is_dir() {
                        if recursive {
                            fs::remove_dir_all(path)?;
                        } else {
                            fs::remove_dir(path)?;
                        }
                        self.write_status(Status::Okay)?;
                    } else {
                        self.write_status(Status::NotDir)?;
                    }
                }
                Command::Presence => {
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

    /// Return a valid path from server root
    fn to_server_path(&self, path: &str) -> String {
        let root = self.root.clone().unwrap();
        let root_str = ::common::beautify_path(&root.display().to_string());
        let cwd = ::common::beautify_path(&self.cwd);
        let mut path_str = if path.starts_with("/") {
            let path_str = format!("{}/{}", root_str, path);
            ::common::canonicalize(&path_str)
        } else {
            let path_str = format!("{}/{}/{}", root_str, cwd, path);
            ::common::canonicalize(&path_str)
        };
        if path_str.contains(&root_str) {
            path_str.drain(root_str.len()..).collect()
        } else {
            "/".to_string()
        }
    }

    /// Return a valid from system root
    fn to_root_path(&self, server_path: &str) -> String {
        format!("{}/{}", self.root.clone().unwrap().display(), server_path)
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
