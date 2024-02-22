use std::{collections::VecDeque, os::unix::fs::symlink, path::PathBuf};

use super::super::DataManager;

impl DataManager {
    pub fn link_theme(&self, theme_path: &PathBuf) {
        let initial_current_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(theme_path).expect("Could not set current directory when recursing into theme directory");
        
        self.recurse_theme(theme_path);
        
        std::env::set_current_dir(initial_current_dir).expect("Could not set current directory when recursing into theme directory");
    } 

    pub fn recurse_theme(&self, theme_path: &PathBuf) {
        // This function recursively symlinks all files from theme into config directory

        if theme_path.is_dir() {
            let children = theme_path.read_dir().unwrap();

            for child in children {
                let child_path = child.expect("Somehow could not read file while recursing into theme directory").path(); 

                // If a child is a directory, then it is also recursed
                if child_path.is_dir() {
                    self.check_make_dir(&child_path);
                    self.recurse_theme(&child_path);
                }
                
                // If a child is a file, then it is symlinked
                else {
                    self.link_file_to_config(&child_path);
                }
            }
        }
    }

    fn check_make_dir(&self, dir_path: &PathBuf) {
        // fix relative
        let relative = make_relative(&dir_path);
        
        let mut final_path = self.config_path.clone();
        final_path = final_path.join(&relative);
        
        if !final_path.exists() {
            std::fs::create_dir_all(final_path).expect("Could not create dir!");
        }
    }

    fn link_file_to_config(&self, file_path: &PathBuf) {
        // fix relative
        let relative = make_relative(&file_path);
        
        let mut link_path = self.config_path.clone();
        link_path = link_path.join(&relative);
        
        symlink(file_path, &link_path).expect(format!("Could not symlink file {} into {}", file_path.display(), link_path.display()).as_str());
    }
}

fn make_relative(path: &PathBuf) -> PathBuf {
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
