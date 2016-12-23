use error::*;
use std::net::{TcpStream, ToSocketAddrs};

pub struct SoftClient {
    // TODO add multiple stream support
    stream: TcpStream,
}

impl SoftClient {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<SoftClient> {
        let stream = TcpStream::connect(addr)?;
        Ok(SoftClient { stream: stream })
    }
}
