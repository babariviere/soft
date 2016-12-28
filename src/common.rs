use error::*;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

/// Receive file from stream
pub fn recv_file<R: Read>(stream: &mut R) -> Result<Vec<u8>> {
    let size = read_size(stream)? as usize;
    let mut data = Vec::with_capacity(size);
    let mut buf = [0; 100];
    let mut read_size = 0;
    while read_size < size {
        let to_read = size - read_size;
        let readed = if to_read <= 100 {
            stream.read(&mut buf[0..to_read])?
        } else {
            stream.read(&mut buf)?
        };
        read_size += readed;
        data.extend_from_slice(&buf[0..readed]);
    }
    Ok(data)
}

/// Receive list of files from stream
pub fn recv_list_file<R: Read>(stream: &mut R) -> Result<Vec<String>> {
    let size = read_size(stream)?;
    let mut list = Vec::new();
    for _ in 0..size {
        let mut buf = String::new();
        read_line(stream, &mut buf)?;
        list.push(buf);
    }
    Ok(list)
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

/// Send list of files to stream
pub fn send_list_file<W: Write>(stream: &mut W, path_name: &str) -> Result<()> {
    let mut list = Vec::new();
    let path = Path::new(path_name);
    let path_name = beautify_path(path_name);
    if path.is_dir() {
        for entry in path.read_dir()? {
            let entry = entry?;
            let file_name = entry.file_name();
            let file_name = file_name.to_str().unwrap();
            list.push(format!("{}/{}", path_name, file_name));
        }
    } else {
        list.push(path_name);
    }
    stream.write(&u64_as_bytes(list.len() as u64))?;
    for file in list {
        let to_send = format!("{}\n", file);
        stream.write(to_send.as_bytes())?;
    }
    Ok(())
}

/// Remove slash in double and remove slash at ends
fn beautify_path(path: &str) -> String {
    let path = path.to_owned();
    let mut new_path = String::new();
    let mut last_char = ' ';
    for c in path.chars() {
        if last_char == '/' && c == '/' {
            continue;
        }
        new_path.push(c);
        last_char = c;
    }
    if new_path.ends_with('/') {
        new_path.pop();
    }
    new_path.replace("./", "")
}

/// Receive size of file or else that will be sent
fn read_size<R: Read>(stream: &mut R) -> Result<u64> {
    let mut buf = [0; 8];
    stream.read(&mut buf)?;
    let mut size: u64 = 0;
    for x in &buf {
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
        if c == '\n' {
            return Ok(());
        }
        buf.push(c);
    }
}

/// Convert an u64 to an array of u8
pub fn u64_as_bytes(num: u64) -> [u8; 8] {
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
