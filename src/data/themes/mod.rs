use unwrap_display::UnwrapDisplay;
use serde::Deserialize;

use super::DataManager;
use super::data_errors::DataError;

use std::fs;
use std::path::PathBuf;

mod link;
mod input;

const TOML_NAME: &str = "Cham.toml";

#[derive(Default)]
pub struct Theme {
    path: PathBuf,
    data: Data,
}

#[derive(Deserialize, Default, Clone)]
#[serde(default)]
pub struct Data {
    pub config: String,
    pub scripts: String,

    pub wallpaper: String,
    pub showcase: String,
}

impl Theme {
    pub fn join_path(&self, path: &str) -> PathBuf{
        self.path.join(path)
    }
}

impl DataManager {
    pub fn set_theme(&self, str: &str) {
        let theme_path = self.themes_path.join(str);
        Self::check_theme(&theme_path).unwrap_display();
        
        let theme = Self::get_theme(&theme_path);
        self.link_theme(&theme);

    }

    fn check_theme(theme_path: &PathBuf) -> Result<(), DataError> {
        if !(theme_path.exists() || theme_path.is_file()) {
             return Err(DataError::ThemeNotFound {theme_name: String::from(theme_path.file_name().expect("Couldnt get file name of theme").to_str().unwrap())})
        }
        if !(theme_path.join(TOML_NAME).exists()) {
            return Err(DataError::NoToml { theme_name: String::from(theme_path.file_name().unwrap().to_str().unwrap()) })
        }
        Ok(())
    }

    fn get_theme(theme_path: &PathBuf) -> Theme{
        let toml_text = fs::read_to_string(theme_path.join(TOML_NAME)).unwrap_display();
        
        let data: Data = toml::from_str(&toml_text).unwrap();
        Theme {
            data,
            path: theme_path.clone(),
        }
    }
}


