use error::*;
use std::io::{Read, Write};
use types::*;

pub struct SoftServer<S: Read + Write> {
    stream: S,
}

impl<S: Read + Write> SoftServer<S> {
    /// Initialize a new server from stream
    pub fn new(stream: S) -> SoftServer<S> {
        SoftServer { stream: stream }
    }

    /// Receive file from client
    pub fn recv_file(&mut self) -> Result<Vec<u8>> {
        ::common::recv_file(&mut self.stream)
    }

    /// Send file to client
    pub fn send_file(&mut self, path: &str) -> Result<()> {
        ::common::send_file(&mut self.stream, path)
    }

    /// Send list of file
    pub fn send_list_file(&mut self, path: &str) -> Result<()> {
        ::common::send_list_file(&mut self.stream, path)
    }

    /// Read command sended by client
    pub fn read_command(&mut self) -> Result<Command> {
        let mut buf = String::new();
        ::common::read_line(&mut self.stream, &mut buf)?;
        Command::try_from(buf)
    }

    /// Write status to client
    pub fn write_status(&mut self, status: Status) -> Result<()> {
        let status = status as u8;
        self.stream.write(&[status])?;
        Ok(())
    }
}
