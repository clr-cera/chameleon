use std::{os::unix::fs::symlink, path::PathBuf};

use super::super::DataManager;
use super::{DataError, Theme};

impl DataManager {
    pub fn link_theme(&self, theme: &Theme) {
        // This functions links theme files into all respective directories
        let data = theme.data.clone();

        if data.config.len() > 0 {
            self.link_directory(&theme.join_path(&data.config), &self.config_path);
        }
        if data.scripts.len() > 0 {
            self.link_directory(&theme.join_path(&data.scripts), &self.scripts_path);
        }
        if data.bin.len() > 0 {
            self.link_directory(&theme.join_path(&data.bin), &self.bin_path);
        }
    }

    fn link_directory(&self, dir_path: &PathBuf, target_path: &PathBuf) {
         // This function symlinks all files from a directory into another
        let initial_current_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(dir_path).expect(DataError::CurrentDirChange { path: dir_path.clone() }.to_string().as_str());
        
        self.link_recurse_directory(dir_path, target_path);
        
        std::env::set_current_dir(&initial_current_dir).expect(DataError::CurrentDirChange { path: initial_current_dir.clone() }.to_string().as_str());
    }

    fn link_recurse_directory(&self, theme_path: &PathBuf, target_path: &PathBuf) {
        // This function recursively symlinks all files from a directory into target

        if theme_path.is_dir() {
            let children = theme_path.read_dir().unwrap();

            for child in children {
                let child_path = child.expect("Somehow could not read file while recursing into theme directory").path(); 

                // If a child is a directory, then it is also recursed
                if child_path.is_dir() {
                    self.link_recurse_directory(&child_path, target_path);
                }
                
                // If a child is a file, then it is symlinked
                else {
                    self.link_file_to_path(&child_path, target_path);
                }
            }
        }
    }


    fn link_file_to_path(&self, file_path: &PathBuf, target_path: &PathBuf) {
        // This function links a single file to target directory with the same relative path as of
        // the current directory

        let relative = Self::make_relative(&file_path);
        
        let link_path = target_path.join(&relative);
       
        if link_path.exists() || link_path.is_symlink() { self.backup_file(&link_path, target_path);}

        Self::ensure_parent(&link_path);

        symlink(file_path, &link_path).expect(format!("{}", DataError::WriteAccess { path: link_path.into() }).as_str());
    }


}

