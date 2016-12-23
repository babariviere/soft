use error::*;
use std::net::{TcpListener, ToSocketAddrs};

pub struct SoftServer {
    listener: TcpListener,
}

impl SoftServer {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> Result<SoftServer> {
        let listener = TcpListener::bind(addr)?;
        Ok(SoftServer { listener: listener })
    }
}
