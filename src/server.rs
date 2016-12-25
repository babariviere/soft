use error::*;
use std::io::{Read, Write};
use std::marker::PhantomData;
use types::Command;

pub struct SoftServer<'a, S: Read + Write + 'a>
    where &'a S: Read
{
    stream: S,
    _marker: PhantomData<&'a ()>,
}

impl<'a, S: Read + Write + 'a> SoftServer<'a, S>
    where &'a S: Read
{
    /// Initialize a new server from stream
    pub fn new(stream: S) -> SoftServer<'a, S> {
        SoftServer {
            stream: stream,
            _marker: PhantomData,
        }
    }

    /// Receive file from client
    pub fn recv_file(&mut self) -> Result<Vec<u8>> {
        ::common::recv_file(&mut self.stream)
    }

    /// Send file to client
    pub fn send_file(&mut self, path: &str) -> Result<()> {
        ::common::send_file(&mut self.stream, path)
    }

    /// Read command sended by client
    pub fn read_command(&mut self) -> Result<Command> {
        let mut buf = String::new();
        ::common::read_line(&mut self.stream, &mut buf)?;
        Command::try_from(buf)
    }
}
