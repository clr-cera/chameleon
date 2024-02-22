use std::path::PathBuf;
use std::collections::VecDeque;
use std::fs;

use super::DataManager;
use super::data_errors::DataError;

impl DataManager {
    fn ensure_dir(&self, dir_path: &PathBuf) {
        // fix relative
        let relative = Self::make_relative(&dir_path);
        
        let mut final_path = self.config_path.clone();
        final_path = final_path.join(&relative);
       
        self.check_and_create_dir(&final_path);
    }
    
    pub fn make_relative(path: &PathBuf) -> PathBuf {
        let current_dir = std::env::current_dir().unwrap();

        let mut absolute = path.clone();
        let mut relative = PathBuf::new();
        let mut components: VecDeque<String> = VecDeque::new();

        loop {
            if absolute == current_dir {break}
            
            components.push_front(String::from(absolute.file_name().unwrap().to_str().unwrap()));
            absolute = absolute.parent().unwrap().into();
        }

        for comp in components {
            relative.push(comp.clone())
        }

        relative
    }

    pub fn ensure_parent(file_path: &PathBuf) {
        let parent = file_path.parent().unwrap();

        if !parent.exists() {
            fs::create_dir_all(parent).expect(format!("{}", DataError::WriteAccess { path: parent.into() }).as_str());
        }
    }
    
    pub fn backup_file(&self, file_path: &PathBuf) {
        let initial_current_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&self.config_path).expect("Could not set current directory when recursing into theme directory");
        
        let backup_file_path = self.backup_path.join(Self::make_relative(file_path));
        
        std::env::set_current_dir(&initial_current_dir).expect("Could not set current directory when recursing into theme directory");

        Self::ensure_parent(&backup_file_path);
        fs::rename(&file_path, &backup_file_path).expect(format!("{}",DataError::WriteAccess { path: backup_file_path.into() }).as_str());
    }
}
