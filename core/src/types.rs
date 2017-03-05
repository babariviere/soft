//! Contains all types used by soft
use error::*;
use std::fmt;

/// All soft commands
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    /// Login to server
    Login(String, String),
    /// Get a file
    Get(String),
    /// Put a file
    Put(String),
    /// List directory
    List(String),
    /// Get current working dir
    Cwd,
    /// Change directory
    Cd(String),
    /// Make directory
    Mkdir(String),
    /// Remove file
    Rm(String),
    /// Remove directory
    Rmdir(String, bool),
    /// Presence check
    Presence,
    /// Exit
    Exit,
}

impl Command {
    /// Try converting string to command
    pub fn try_from<S: AsRef<str>>(s: S) -> Result<Command> {
        let s = s.as_ref().to_string();
        let splitted = s.split_whitespace().map(|s| s.to_owned()).collect::<Vec<String>>();
        match splitted[0].as_str() {
            "LOGIN" => {
                if splitted.len() != 3 {
                    bail!(ErrorKind::InvalidCommand(s));
                }
                Ok(Command::Login(splitted[1].clone(), splitted[2].clone()))
            }
            "GET" => {
                if splitted.len() != 2 {
                    bail!(ErrorKind::InvalidCommand(s));
                }
                Ok(Command::Get(splitted[1].clone()))
            }
            "PUT" => {
                if splitted.len() != 2 {
                    bail!(ErrorKind::InvalidCommand(s));
                }
                Ok(Command::Put(splitted[1].clone()))
            }
            "LIST" => {
                if splitted.len() != 2 {
                    bail!(ErrorKind::InvalidCommand(s));
                }
                Ok(Command::List(splitted[1].clone()))
            }
            "CWD" => Ok(Command::Cwd),
            "CD" => {
                if splitted.len() != 2 {
                    bail!(ErrorKind::InvalidCommand(s));
                }
                Ok(Command::Cd(splitted[1].clone()))
            }
            "MKDIR" => {
                if splitted.len() != 2 {
                    bail!(ErrorKind::InvalidCommand(s));
                }
                Ok(Command::Mkdir(splitted[1].clone()))
            }
            "RM" => {
                if splitted.len() != 2 {
                    bail!(ErrorKind::InvalidCommand(s));
                }
                Ok(Command::Rm(splitted[1].clone()))
            }
            "RMDIR" => {
                if splitted.len() != 3 {
                    bail!(ErrorKind::InvalidCommand(s));
                }
                Ok(Command::Rmdir(splitted[1].clone(),
                                  splitted[2].clone().parse::<bool>().unwrap()))
            }
            "PRESENCE" => Ok(Command::Presence),
            "EXIT" => Ok(Command::Exit),
            _ => bail!(ErrorKind::InvalidCommand(s)),
        }
    }

    /// Get login username and password
    /// Only work for Login, else it will panic
    pub fn unwrap_login(self) -> (String, String) {
        match self {
            Command::Login(u, p) => (u, p),
            c => panic!("Command \'{}\' doesn't contain login information", c),
        }
    }

    /// Get the path from command,
    /// Work for Get, Put and List, else it will panic
    pub fn unwrap_path(self) -> String {
        match self {
            Command::Get(s) |
            Command::Put(s) |
            Command::List(s) |
            Command::Cd(s) |
            Command::Rm(s) |
            Command::Rmdir(s, _) |
            Command::Mkdir(s) => s,
            c => panic!("Command \'{}\' doesn't contain path", c),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::Login(ref u, ref p) => write!(f, "LOGIN {} {}", u, p),
            Command::Get(ref p) => write!(f, "GET {}", p),
            Command::Put(ref p) => write!(f, "PUT {}", p),
            Command::List(ref p) => write!(f, "LIST {}", p),
            Command::Cwd => write!(f, "CWD"),
            Command::Cd(ref p) => write!(f, "CD {}", p),
            Command::Mkdir(ref p) => write!(f, "MKDIR {}", p),
            Command::Rm(ref p) => write!(f, "RM {}", p),
            Command::Rmdir(ref p, ref r) => write!(f, "RMDIR {} {}", p, r),
            Command::Presence => write!(f, "PRESENCE"),
            Command::Exit => write!(f, "EXIT"),
        }
    }
}

/// Status of command
pub enum Status {
    /// Connected to server
    Connected = 1,
    /// Disconnected from server
    Disconnected = 2,
    /// Wrong login credential
    WrongLogin = 3,
    /// Not connected to server
    NotConnected = 4,
    /// Okay
    Okay = 5,
    /// Not a file
    NotFile = 6,
    /// Not a directory
    NotDir = 7,
    /// Unknown path
    PathUnknown = 8,
    /// Unknown error
    UnkownError = 255,
}

impl Status {
    /// Check if status is positive
    pub fn is_positive(&self) -> bool {
        match *self {
            Status::Connected | Status::Disconnected | Status::Okay => true,
            _ => false,
        }
    }

    /// Check if status is negative
    pub fn is_negative(&self) -> bool {
        !self.is_positive()
    }
}

impl From<u8> for Status {
    fn from(from: u8) -> Status {
        match from {
            1 => Status::Connected,
            2 => Status::Disconnected,
            3 => Status::WrongLogin,
            4 => Status::NotConnected,
            5 => Status::Okay,
            6 => Status::NotFile,
            7 => Status::NotDir,
            8 => Status::PathUnknown,
            _ => Status::UnkownError,
        }
    }
}
