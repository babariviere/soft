use error::*;
use std::io::{Read, Write};
use types::Command;

pub struct SoftClient<S: Read + Write> {
    stream: S,
}

impl<S: Read + Write> SoftClient<S> {
    /// Initialize a new client from stream
    pub fn new(stream: S) -> SoftClient<S> {
        SoftClient { stream: stream }
    }

    /// Connect to soft server
    pub fn connect(&mut self, _user: &str, _pass: &str) -> Result<()> {
        unimplemented!()
    }

    /// Ask and get file from soft server
    pub fn get(&mut self, path: &str) -> Result<Vec<u8>> {
        self.write_command(Command::Get(path.into()))?;
        self.recv_file()
    }

    // Low level functions

    /// Send a command to server
    /// Warning: this is a low level function
    pub fn write_command(&mut self, command: Command) -> Result<()> {
        self.stream.write(format!("{}\n", command).as_bytes())?;
        Ok(())
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
