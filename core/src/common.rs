//! Common module contains all function needed by server and client
use error::*;
use std::fs;
use std::io::{Read, Write};
use std::mem;
use std::path::Path;

/// Receive file from stream
pub fn recv_file<R: Read>(stream: &mut R) -> Result<Vec<u8>> {
    let size = read_size(stream)? as usize;
    let mut data = Vec::with_capacity(size);
    let mut buf = [0; 100];
    let mut read_size = 0;
    while read_size < size {
        let to_read = size - read_size;
        let readed = if to_read < 100 {
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
pub fn send_list_file<W: Write>(stream: &mut W, list: Vec<String>) -> Result<()> {
    stream.write(&u64_as_bytes(list.len() as u64))?;
    for file in list {
        let to_send = format!("{}\n", file);
        stream.write(to_send.as_bytes())?;
    }
    Ok(())
}

/// Remove slash in double and remove slash at ends
pub fn beautify_path(path: &str) -> String {
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

/// Return an absolute path of string
pub fn canonicalize<P: AsRef<Path>>(path: P) -> String {
    let path = path.as_ref().to_str().unwrap_or(".");
    let path = if path.starts_with('/') {
        path.to_owned()
    } else {
        let cur = ::std::env::current_dir().unwrap().display().to_string();
        format!("{}/{}", cur, path)
    };
    let path_tmp = path.split('/')
        .filter(|p| *p != "." && !p.is_empty())
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    let mut path = Vec::new();
    for i in 0..path_tmp.len() {
        if path_tmp[i] == ".." {
            path.pop();
        } else {
            path.push(path_tmp[i].clone());
        }
    }
    path.iter().map(|s| format!("/{}", s)).collect::<String>()
}

/// Receive size of file or else that will be sent
fn read_size<R: Read>(stream: &mut R) -> Result<u64> {
    let mut buf = [0; 8];
    stream.read(&mut buf)?;
    let size: u64 = bytes_to_u64(buf);
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
        ::std::thread::sleep(::std::time::Duration::from_millis(50));
    }
}

/// Convert an u64 to an array of u8
pub fn u64_as_bytes(num: u64) -> [u8; 8] {
    unsafe { mem::transmute::<u64, [u8; 8]>(num) }
}

/// Convert an array of u8 to u64
pub fn bytes_to_u64(arr: [u8; 8]) -> u64 {
    unsafe { mem::transmute::<[u8; 8], u64>(arr) }
}
