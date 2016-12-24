use error::*;
use std::io::{Read, Write};

pub struct SoftServer<S: Read + Write>(S);

impl<S: Read + Write> SoftServer<S> {
    pub fn new(stream: S) -> SoftServer<S> {
        SoftServer(stream)
    }
}
