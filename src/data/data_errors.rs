use std::error::Error;
use std::fmt;
use std::path::PathBuf;


#[derive(Debug)]
pub enum DataError {
    ThemeNotFound {theme_name: String}, 
    NoToml {theme_name: String},
    WriteAccess {path: PathBuf},
    CurrentDirChange {path: PathBuf},
    NoConnection {url: String},
    WrongExtension {path: PathBuf, ext: String},
    OSNotSupported,
}

impl Error for DataError {}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ThemeNotFound { theme_name } => write!(f, "There is no theme named {theme_name}"),

            Self::NoToml { theme_name } => write!(f, "Theme {theme_name} does not have a Cham.toml file"),

            Self::WriteAccess { path } => write!(f, "User does not have access to write on {}", path.display()),
            
            Self::CurrentDirChange { path } => write!(f, "Could not change current dir into {}", path.display()),

            Self::NoConnection { url } => write!(f, "Could not connect to server. Url: {url}"),
            
            Self::WrongExtension { path, ext } => write!(f, "{} has wrong extension, should be {ext}", path.display()),

            Self::OSNotSupported => write!(f, "The operational system is not supported by this application"),
        }

    }
}

