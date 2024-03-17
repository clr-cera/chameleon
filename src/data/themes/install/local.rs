use std::env::current_dir;
use std::path::PathBuf;

use crate::DataManager;

impl DataManager {
    pub fn local_install(&self, path: &str) {
        let mut local_path = PathBuf::from(path);

        if local_path.is_relative() {
            local_path = current_dir().unwrap().join(local_path);
        }


        if local_path.is_file(){
            let new_theme_path = self.themes_path.join("New");
            Self::extract_zip(&local_path, &new_theme_path);
        }

        else if local_path.is_dir(){
            Self::copy_folder(&local_path, &self.themes_path);
        }
    }

    fn copy_folder(origin_path: &PathBuf, target_path: &PathBuf) {
        // This function recursively moves all files from origin into target
        Self::copy_folder_rec(origin_path, target_path, &origin_path.clone())

    }

    fn copy_folder_rec(folder_path: &PathBuf, target_path: &PathBuf, dir_path: &PathBuf) {
        if folder_path.is_dir() {
            let children = folder_path.read_dir().unwrap();

            for child in children {
                let child_path = child.expect("Somehow could not read file while recursing into theme directory").path(); 

                // If a child is a directory, then it is also recursed
                if child_path.is_dir() {
                    Self::copy_folder_rec(&child_path, target_path, dir_path);
                }

                // If a child is a file, then it is moved
                else {
                    Self::copy_file(target_path, &child_path, dir_path);
                }
            }
        }
    }
}
