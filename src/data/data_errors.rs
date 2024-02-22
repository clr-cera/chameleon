use std::error::Error;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum DataError {
    ThemeNotFound {theme_name: String}, 
    WriteAccess {path: PathBuf},
    CurrentDirChange {path: PathBuf},
}

impl Error for DataError {}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ThemeNotFound { theme_name } => write!(f, "There is no theme named {}", theme_name),

            Self::WriteAccess { path } => write!(f, "User does not have access to write on {}", path.display()),
            
            Self::CurrentDirChange { path } => write!(f, "Could not change current dir into {}", path.display()),
        }

    }
}
