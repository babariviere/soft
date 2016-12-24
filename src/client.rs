use error::*;
use std::io::{Read, Write};
use types::Command;

pub struct SoftClient<S: Read + Write> {
    stream: S,
}

impl<S: Read + Write> SoftClient<S> {
    pub fn new(stream: S) -> SoftClient<S> {
        SoftClient { stream: stream }
    }

    pub fn write_command(&mut self, command: Command) -> Result<()> {
        self.stream.write(format!("{}\n", command).as_bytes())?;
        Ok(())
    }
}
