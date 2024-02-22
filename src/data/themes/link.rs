use std::{os::unix::fs::symlink, path::PathBuf};

use super::super::DataManager;
use super::DataError;

impl DataManager {
    pub fn link_theme(&self, theme_path: &PathBuf) {
        let initial_current_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&theme_path).expect(DataError::CurrentDirChange { path: theme_path.clone() }.to_string().as_str());
        
        self.recurse_theme(&theme_path);
        
        std::env::set_current_dir(&initial_current_dir).expect(DataError::CurrentDirChange { path: initial_current_dir.clone() }.to_string().as_str());
    } 

    pub fn recurse_theme(&self, theme_path: &PathBuf) {
        // This function recursively symlinks all files from theme into config directory

        if theme_path.is_dir() {
            let children = theme_path.read_dir().unwrap();

            for child in children {
                let child_path = child.expect("Somehow could not read file while recursing into theme directory").path(); 

                // If a child is a directory, then it is also recursed
                if child_path.is_dir() {
                    self.recurse_theme(&child_path);
                }
                
                // If a child is a file, then it is symlinked
                else {
                    self.link_file_to_config(&child_path);
                }
            }
        }
    }


    fn link_file_to_config(&self, file_path: &PathBuf) {
        // fix relative
        let relative = Self::make_relative(&file_path);
        
        let mut link_path = self.config_path.clone();
        link_path = link_path.join(&relative);
        
        if link_path.exists() { self.backup_file(&link_path);}

        Self::ensure_parent(&link_path);

        symlink(file_path, &link_path).expect(format!("{}", DataError::WriteAccess { path: link_path.into() }).as_str());
    }


}

