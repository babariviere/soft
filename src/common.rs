use error::*;
use std::fs;
use std::io::{Read, Write};

/// Receive file from stream
pub fn recv_file<R: Read>(stream: &mut R) -> Result<Vec<u8>> {
    let size = read_file_size(stream)? as usize;
    let mut data = Vec::with_capacity(size);
    let mut buf = [0; 100];
    let mut read_size = 0;
    while read_size < size {
        let readed = stream.read(&mut buf)?;
        read_size += readed;
        data.extend_from_slice(&buf[0..readed]);
    }
    stream.read(&mut buf)?;
    Ok(data)
}

/// Send file to stream
pub fn send_file<W: Write>(stream: &mut W, path: &str) -> Result<()> {
    let mut file = fs::File::open(path)?;
    let size = file.metadata()?.len();
    stream.write(&u64_as_bytes(size))?;
    let mut write_size = 0;
    while write_size < size as usize {
        let mut buf = [0; 100];
        let readed = file.read(&mut buf)?;
        let writed = stream.write(&buf[0..readed])?;
        write_size += writed;
    }
    Ok(())
}

/// Receive size of file that will be sent
fn read_file_size<R: Read>(stream: &mut R) -> Result<u64> {
    let mut buf = [0; 8];
    stream.read(&mut buf)?;
    let mut size: u64 = 0;
    for x in buf.iter() {
        size += *x as u64;
    }
    Ok(size)
}

/// Read line from stream
pub fn read_line<R: Read>(stream: &mut R, buf: &mut String) -> Result<()> {
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
pub fn u64_as_bytes<'a>(num: u64) -> [u8; 8] {
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
