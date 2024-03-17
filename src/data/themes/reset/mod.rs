use crate::data::data_errors::DataError;
use crate::DataManager;
use std::fs;
use std::path::PathBuf;

impl DataManager {
    pub fn reset(&self) {
        self.resetrec(&self.config_path);
        self.resetrec(&self.scripts_path);
    }

    fn resetrec(&self, dir_path: &PathBuf) {
    // This function recursively removes all files that are linked to chameleon data from dir 
        let children = dir_path.read_dir().unwrap();

        for child in children {
            let child_path = child.expect("Somehow could not read file while recursing into theme directory").path(); 

            // If a child is a directory, then it is also recursed
            if child_path.is_dir() {
                self.resetrec(&child_path);
                
                let count = walkdir::WalkDir::new(&child_path).into_iter().count();
                if  count == 1 {
                    fs::remove_dir(&child_path).unwrap();
                }
            }
            
            // If a child is a symlink that points to chameleon data itis
            else if child_path.is_symlink(){
                if fs::read_link(&child_path).unwrap().starts_with(&self.data_path) {
                    fs::remove_file(&child_path).expect(DataError::WriteAccess { path: child_path }.to_string().as_str());
                }
                
            }
        }
    }
}
