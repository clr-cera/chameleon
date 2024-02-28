use unwrap_display::UnwrapDisplay;

use super::DataManager;
use super::data_errors::DataError;

use std::path::PathBuf;

mod link;
mod input;

impl DataManager {
    pub fn set_theme(self, str: &str) {
        let theme_path = self.themes_path.join(str);
        
        Self::check_theme(&theme_path).unwrap_display();

        self.link_theme(&theme_path);

    }

    fn check_theme(theme_path: &PathBuf) -> Result<(), DataError> {
        if !(theme_path.exists() || theme_path.is_file()) {
             return Err(DataError::ThemeNotFound {theme_name: String::from(theme_path.file_name().expect("Couldnt get file name of theme").to_str().unwrap())})
        }

        Ok(())
    }
}


