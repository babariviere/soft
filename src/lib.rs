#[macro_use]
extern crate error_chain;

pub mod client;
pub mod error;
pub mod server;
pub mod types;

pub const DEFAULT_PORT: u32 = 9045;

#[cfg(test)]
mod tests {
    use client::SoftClient;
    use server::SoftServer;
    use std::fs::OpenOptions;
    use types::*;

    #[test]
    fn file_stream() {
        let server_stream =
            OpenOptions::new().read(true).write(true).create(true).open(".file_stream").unwrap();
        let client_stream = OpenOptions::new().read(true).write(true).open(".file_stream").unwrap();
        let mut server = SoftServer::new(server_stream);
        let mut client = SoftClient::new(client_stream);
        client.write_command(Command::Exit).unwrap();
        assert_eq!(server.read_command().unwrap(), Command::Exit);
        ::std::fs::remove_file(".file_stream").unwrap();
    }
}
