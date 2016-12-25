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
        self.get_file()
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
    pub fn get_file(&mut self) -> Result<Vec<u8>> {
        let size = self.read_file_size()? as usize;
        let mut data = Vec::with_capacity(size);
        let mut buf = [0; 100];
        let mut read_size = 0;
        while read_size < size {
            let readed = self.stream.read(&mut buf)?;
            read_size += readed;
            data.extend_from_slice(&buf[0..readed]);
        }
        self.stream.read(&mut buf)?;
        Ok(data)
    }

    /// Receive size of file that will be sent
    fn read_file_size(&mut self) -> Result<u64> {
        let mut buf = [0; 8];
        self.stream.read(&mut buf)?;
        let mut size: u64 = 0;
        for x in buf.iter() {
            size += *x as u64;
        }
        Ok(size)
    }
}
