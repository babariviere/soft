
error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links {}

    foreign_links {
        Io(::std::io::Error);
        AppDirs(::app_dirs::AppDirsError);
    }

    errors {
        InvalidCommand(c: String) {
            description("parsed command is invalid")
            display("invalid command: {}", c)
        }
        InvalidLogin {
            description("user name or password is invalid")
            display("user name or password is invalid")
        }
        NotConnected {
            description("client is not logged in server")
            display("client is not logged in server")
        }
        InvalidUserDB {
            description("provided user database is invalid")
            display("provided user database is invalid")
        }
    }
}
