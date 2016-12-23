use error::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Login(String, String),
    Get(String),
    Put(String),
    List(String),
    Exit,
}

impl Command {
    pub fn try_from<S: AsRef<str>>(s: S) -> Result<Command> {
        let s = s.as_ref().to_string();
        let clone = s.clone();
        let len = clone.split_whitespace().count();
        let mut splitted = clone.split_whitespace();
        match splitted.next().unwrap() {
            "LOGIN" => {
                if len != 3 {
                    bail!(ErrorKind::InvalidCommand(s));
                }
                Ok(Command::Login(splitted.next().unwrap().to_string(),
                                  splitted.next().unwrap().to_string()))
            }
            "GET" => {
                if len != 2 {
                    bail!(ErrorKind::InvalidCommand(s));
                }
                Ok(Command::Get(splitted.next().unwrap().to_string()))
            }
            "PUT" => {
                if len != 2 {
                    bail!(ErrorKind::InvalidCommand(s));
                }
                Ok(Command::Put(splitted.next().unwrap().to_string()))
            }
            "LIST" => {
                if len != 2 {
                    bail!(ErrorKind::InvalidCommand(s));
                }
                Ok(Command::List(splitted.next().unwrap().to_string()))
            }
            "EXIT" => Ok(Command::Exit),
            _ => bail!(ErrorKind::InvalidCommand(s)),
        }
    }
}

impl ToString for Command {
    fn to_string(&self) -> String {
        match *self {
            Command::Login(ref u, ref p) => format!("LOGIN {} {}", u, p),
            Command::Get(ref p) => format!("GET {}", p),
            Command::Put(ref p) => format!("PUT {}", p),
            Command::List(ref p) => format!("LIST {}", p),
            Command::Exit => "EXIT".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use command::Command;

    #[test]
    fn command_from_str() {
        assert_eq!(Command::try_from("LOGIN user pass").unwrap(),
                   Command::Login("user".into(), "pass".into()));
        assert_eq!(Command::try_from("GET /path").unwrap(),
                   Command::Get("/path".into()));
        assert_eq!(Command::try_from("PUT /path").unwrap(),
                   Command::Put("/path".into()));
        assert_eq!(Command::try_from("LIST /path").unwrap(),
                   Command::List("/path".into()));
        assert_eq!(Command::try_from("EXIT").unwrap(), Command::Exit);
        assert!(Command::try_from("LOGIN BLA").is_err());
        assert!(Command::try_from("GET hehe hehe").is_err());
        assert!(Command::try_from("PUT path path2").is_err());
        assert!(Command::try_from("LIST p p").is_err());
        assert!(Command::try_from("login user pass").is_err());
    }

    #[test]
    fn command_to_str() {
        assert_eq!(Command::Login("user".into(), "pass".into()).to_string(),
                   "LOGIN user pass");
        assert_eq!(Command::Get("/path".into()).to_string(), "GET /path");
        assert_eq!(Command::Put("/path".into()).to_string(), "PUT /path");
        assert_eq!(Command::List("/path".into()).to_string(), "LIST /path");
        assert_eq!(Command::Exit.to_string(), "EXIT");
    }
}
