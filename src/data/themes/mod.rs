use unwrap_display::UnwrapDisplay;
use serde::Deserialize;

use super::DataManager;
use super::data_errors::DataError;
use super::binaries::BinManager;
use std::fs;
use std::path::PathBuf;

mod link;
mod install;
mod reset;

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
    pub bin: String,

    pub wallpaper: String,
    pub showcase: String,

    pub name: String,
    pub description: String
}

impl Theme {
    pub fn join_path(&self, path: &str) -> PathBuf{
        self.path.join(path)
    }
}

impl DataManager {
    pub fn set_theme(&self, theme_name: &str) {
        let theme = self.find_theme(theme_name).expect(DataError::ThemeNotFound { theme_name: theme_name.to_string() }.to_string().as_str());
        self.link_theme(&theme);

        Self::make_commands(&theme);
    }

    fn check_theme(theme_path: &PathBuf, theme_name: &str) -> bool {
        if theme_path.join(TOML_NAME).exists() {
            let toml_text = fs::read_to_string(theme_path.join(TOML_NAME)).unwrap_display();
            let data: Data = toml::from_str(&toml_text).unwrap();
            if data.name == theme_name {
                return true;
            }
        }
        false
    }

    fn get_theme(theme_path: &PathBuf) -> Theme{
        let toml_text = fs::read_to_string(theme_path.join(TOML_NAME)).unwrap_display();
        
        let data: Data = toml::from_str(&toml_text).unwrap();
        Theme {
            data,
            path: theme_path.clone(),
        }
    }

    fn find_theme(&self, theme_name: &str) -> Option<Theme>{
        //This function iterates through all themes and finds one which has the same name as
        //informed
        for theme in self.themes_path.read_dir().unwrap() {
            let theme_path = theme.unwrap().path();
            if Self::check_theme(&theme_path, theme_name) {
                return Some(Self::get_theme(&theme_path));
            }
        }

        None
    }

    fn make_commands(theme: &Theme) {
        if theme.data.wallpaper.len() > 0 {
            BinManager::set_wallpaper(&theme.join_path(&theme.data.wallpaper))
        }
    }
}


