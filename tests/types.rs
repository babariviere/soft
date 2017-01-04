extern crate soft;

use soft::types::Command;

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
    assert_eq!(Command::try_from("CWD").unwrap(), Command::Cwd);
    assert_eq!(Command::try_from("CD path").unwrap(),
               Command::Cd("path".into()));
    assert_eq!(Command::try_from("MKDIR path").unwrap(),
               Command::Mkdir("path".into()));
    assert_eq!(Command::try_from("RM path").unwrap(),
               Command::Rm("path".into()));
    assert_eq!(Command::try_from("RMDIR path").unwrap(),
               Command::Rmdir("path".into()));
    assert_eq!(Command::try_from("PRESENCE").unwrap(), Command::Presence);
    assert_eq!(Command::try_from("EXIT").unwrap(), Command::Exit);
    assert!(Command::try_from("LOGIN BLA").is_err());
    assert!(Command::try_from("GET hehe hehe").is_err());
    assert!(Command::try_from("PUT path path2").is_err());
    assert!(Command::try_from("LIST p p").is_err());
    assert!(Command::try_from("CD").is_err());
    assert!(Command::try_from("MKDIR").is_err());
    assert!(Command::try_from("RM").is_err());
    assert!(Command::try_from("RMDIR").is_err());
    assert!(Command::try_from("login user pass").is_err());
}

#[test]
fn command_to_str() {
    assert_eq!(Command::Login("user".into(), "pass".into()).to_string(),
               "LOGIN user pass");
    assert_eq!(Command::Get("/path".into()).to_string(), "GET /path");
    assert_eq!(Command::Put("/path".into()).to_string(), "PUT /path");
    assert_eq!(Command::List("/path".into()).to_string(), "LIST /path");
    assert_eq!(Command::Cwd.to_string(), "CWD");
    assert_eq!(Command::Cd("path".into()).to_string(), "CD path");
    assert_eq!(Command::Mkdir("path".into()).to_string(), "MKDIR path");
    assert_eq!(Command::Rm("path".into()).to_string(), "RM path");
    assert_eq!(Command::Rmdir("path".into()).to_string(), "RMDIR path");
    assert_eq!(Command::Presence.to_string(), "PRESENCE");
    assert_eq!(Command::Exit.to_string(), "EXIT");
}

#[test]
fn command_unwrap_login() {
    assert_eq!(Command::Login("user".into(), "pass".into()).unwrap_login(),
               ("user".into(), "pass".into()));
}

#[test]
#[should_panic]
fn command_unwrap_login_panic() {
    let _ = Command::Exit.unwrap_login();
}

#[test]
fn command_unwrap_path() {
    assert_eq!(Command::Get("/path".into()).unwrap_path(), "/path");
    assert_eq!(Command::Put("/path".into()).unwrap_path(), "/path");
    assert_eq!(Command::List("/path".into()).unwrap_path(), "/path");
    assert_eq!(Command::Cd("path".into()).unwrap_path(), "path");
    assert_eq!(Command::Mkdir("path".into()).unwrap_path(), "path");
    assert_eq!(Command::Rm("path".into()).unwrap_path(), "path");
    assert_eq!(Command::Rmdir("path".into()).unwrap_path(), "path");
}

#[test]
#[should_panic]
fn command_unwrap_path_panic() {
    let _ = Command::Exit.unwrap_path();
}
