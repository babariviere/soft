
error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links {}

    foreign_links {
        Io(::std::io::Error);
    }

    errors {
        InvalidCommand(c: String) {
            description("parsed command is invalid")
            display("invalid command: {}", c)
        }
        NotConnected {
            description("client is not logged in server")
            display("client is not logged in server")
        }
    }
}
