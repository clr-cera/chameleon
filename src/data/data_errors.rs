use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DataError {
    ThemeNotFound {theme_name: String}, 
}

impl Error for DataError {}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ThemeNotFound { theme_name } => write!(f, "There is no theme named {}", theme_name)
        }

    }
}
