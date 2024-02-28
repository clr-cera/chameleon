use std::env::current_dir;
use std::path::PathBuf;

use crate::DataManager;

impl DataManager {
    pub fn local_install(&self, path: &str) {
        let mut zip_path = PathBuf::from(path);

        if zip_path.is_relative() {
            zip_path = current_dir().unwrap().join(zip_path);
        }

        let theme_path = self.themes_path.join("New");

        let _new_theme_path = Self::extract_zip(&zip_path, &theme_path);
    }
}
