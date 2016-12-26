use error::*;
use std::io::{Read, Write};
use types::*;

pub struct SoftClient<S: Read + Write> {
    stream: S,
}

impl<S: Read + Write> SoftClient<S> {
    /// Initialize a new client from stream
    pub fn new(stream: S) -> SoftClient<S> {
        SoftClient { stream: stream }
    }

    /// Login to soft server
    pub fn login(&mut self, user: &str, pass: &str) -> Result<Status> {
        self.write_command(Command::Login(user.into(), pass.into()))?;
        self.read_status()
    }

    /// Ask and get file from soft server
    pub fn get(&mut self, path: &str) -> Result<Vec<u8>> {
        self.write_command(Command::Get(path.into()))?;
        self.recv_file()
    }

    /// Ask and put file to soft server
    pub fn put(&mut self, local_path: &str, remote_path: &str) -> Result<()> {
        self.write_command(Command::Put(remote_path.into()))?;
        self.send_file(local_path)
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

    /// Send file to soft server
    /// Warning: this is a low level function
    pub fn send_file(&mut self, path: &str) -> Result<()> {
        ::common::send_file(&mut self.stream, path)
    }
}
