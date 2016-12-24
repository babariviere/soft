use error::*;
use std::io::{BufRead, BufReader, Read, Write};
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
    pub fn new(stream: S) -> SoftServer<'a, S> {
        SoftServer {
            stream: stream,
            _marker: PhantomData,
        }
    }

    pub fn read_command(&'a mut self) -> Result<Command> {
        let mut buf = String::new();
        let mut bufreader = BufReader::new(&self.stream);
        bufreader.read_line(&mut buf)?;
        Command::try_from(buf)
    }
}
