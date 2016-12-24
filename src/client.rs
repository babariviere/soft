use error::*;
use std::io::{Read, Write};

pub struct SoftClient<S: Read + Write>(S);

impl<S: Read + Write> SoftClient<S> {
    pub fn new(stream: S) -> SoftClient<S> {
        SoftClient(stream)
    }
}
