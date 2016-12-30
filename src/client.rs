use error::*;
use std::io::{Read, Write};
use types::*;

pub struct SoftClient<S: Read + Write> {
    stream: S,
    exited: bool,
}

impl<S: Read + Write> SoftClient<S> {
    /// Initialize a new client from stream
    pub fn new(stream: S) -> SoftClient<S> {
        SoftClient {
            stream: stream,
            exited: false,
        }
    }

    /// Login to soft server
    pub fn login(&mut self, user: &str, pass: &str) -> Result<()> {
        self.write_command(Command::Login(user.into(), pass.into()))?;
        self.check_status()
    }

    /// Ask and get file from soft server
    pub fn get(&mut self, path: &str) -> Result<Vec<u8>> {
        self.write_command(Command::Get(path.into()))?;
        self.check_status()?;
        self.recv_file()
    }

    /// Ask and put file to soft server
    pub fn put(&mut self, local_path: &str, remote_path: &str) -> Result<()> {
        self.write_command(Command::Put(remote_path.into()))?;
        self.check_status()?;
        self.send_file(local_path)
    }

    /// Ask and list file from soft server
    pub fn list(&mut self, path: &str) -> Result<Vec<String>> {
        self.write_command(Command::List(path.into()))?;
        self.check_status()?;
        self.recv_list_file()
    }

    /// Get the current working directory
    pub fn cwd(&mut self) -> Result<String> {
        self.write_command(Command::Cwd)?;
        self.check_status()?;
        self.read_line()
    }

    /// Change directory
    pub fn cd(&mut self, path: &str) -> Result<()> {
        self.write_command(Command::Cd(path.into()))?;
        self.check_status()
    }

    /// Make directory
    pub fn mkdir(&mut self, path: &str) -> Result<()> {
        self.write_command(Command::Mkdir(path.into()))?;
        self.check_status()
    }

    /// Send to server an exit command
    pub fn exit(&mut self) -> Result<()> {
        self.write_command(Command::Exit)?;
        self.exited = true;
        self.check_status()
    }

    /// Check sended status
    fn check_status(&mut self) -> Result<()> {
        let status = self.read_status()?;
        if status.is_negative() {
            match status {
                Status::WrongLogin => bail!(ErrorKind::InvalidLogin),
                Status::NotConnected => bail!(ErrorKind::NotConnected),
                _ => {}
            }
        }
        Ok(())
    }

    // Low level functions

    /// Send a command to server
    /// Warning: this is a low level function
    pub fn write_command(&mut self, command: Command) -> Result<()> {
        self.stream.write(format!("{}\n", command).as_bytes())?;
        Ok(())
    }

    /// Receive status from server
    /// Warning: this is a low level function
    pub fn read_status(&mut self) -> Result<Status> {
        let mut buf = [0];
        self.stream.read(&mut buf)?;
        Ok(Status::from(buf[0]))
    }

    /// Receive file from soft server
    /// Warning: this is a low level function
    pub fn recv_file(&mut self) -> Result<Vec<u8>> {
        ::common::recv_file(&mut self.stream)
    }

    /// Receive list of file from soft server
    /// Warning: this is a low level function
    pub fn recv_list_file(&mut self) -> Result<Vec<String>> {
        ::common::recv_list_file(&mut self.stream)
    }

    /// Send file to soft server
    /// Warning: this is a low level function
    pub fn send_file(&mut self, path: &str) -> Result<()> {
        ::common::send_file(&mut self.stream, path)
    }

    /// Read a single line
    pub fn read_line(&mut self) -> Result<String> {
        let mut buf = String::new();
        ::common::read_line(&mut self.stream, &mut buf)?;
        Ok(buf)
    }
}

impl<S: Read + Write> Drop for SoftClient<S> {
    fn drop(&mut self) {
        if !self.exited {
            let _ = self.exit();
        }
    }
}
