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
                    let root = self.root.clone().unwrap();
                    let path = root.join(p);
                    self.send_list_file(&path.display().to_string())?;
                }
                Command::Cwd => {
                    if self.root.is_none() {
                        self.write_status(Status::NotConnected)?;
                        continue;
                    }
                    self.write_status(Status::Okay)?;
                    self.stream.write(format!("{}\n", self.cwd).as_bytes())?;
                }
                Command::Cd(p) => unimplemented!(),
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
    fn send_list_file(&mut self, path: &str) -> Result<()> {
        ::common::send_list_file(&mut self.stream, path)
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
}
