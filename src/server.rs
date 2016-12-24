use error::*;
use std::fs;
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

    /// Send file to client
    pub fn send_file(&mut self, path: &str) -> Result<()> {
        let mut file = fs::File::open(path)?;
        let size = file.metadata()?.len();
        self.stream.write(&u64_as_bytes(size))?;
        let mut write_size = 0;
        while write_size < size as usize {
            let mut buf = [0; 100];
            file.read(&mut buf)?;
            let writed = self.stream.write(&buf)?;
            write_size += writed;
        }
        Ok(())
    }

    /// Read command sended by client
    pub fn read_command(&mut self) -> Result<Command> {
        let mut buf = String::new();
        read_line(&mut self.stream, &mut buf)?;
        Command::try_from(buf)
    }
}

fn read_line<R: Read>(stream: &mut R, buf: &mut String) -> Result<()> {
    let mut b = [0];
    loop {
        stream.read(&mut b)?;
        let c = b[0] as char;
        buf.push(c);
        if c == '\n' {
            return Ok(());
        }
    }
}

/// Convert an u64 to an array of u8
fn u64_as_bytes<'a>(num: u64) -> [u8; 8] {
    let mut arr = [0; 8];
    arr[0] = (num >> 56) as u8;
    arr[1] = ((num >> 48) - ((arr[0] as u64) << 8)) as u8;
    arr[2] = ((num >> 40) - ((arr[1] as u64) << 8)) as u8;
    arr[3] = ((num >> 32) - ((arr[2] as u64) << 8)) as u8;
    arr[4] = ((num >> 24) - ((arr[3] as u64) << 8)) as u8;
    arr[5] = ((num >> 16) - ((arr[4] as u64) << 8)) as u8;
    arr[6] = ((num >> 8) - ((arr[5] as u64) << 8)) as u8;
    arr[7] = (num - ((arr[6] as u64) << 8)) as u8;
    arr
}
