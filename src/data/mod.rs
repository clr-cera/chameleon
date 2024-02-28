use std::{fs, path::PathBuf};

use dirs::{data_dir, config_dir};

mod themes;
mod data_errors;
mod data_lib;

use tempdir::TempDir;

pub struct DataManager {
    data_path: PathBuf,
    themes_path: PathBuf,
    backup_path: PathBuf,
    config_path:PathBuf,
    tmp_dir: TempDir,
}

impl DataManager {
    pub fn new() -> Self{
        let data_path = {
            let mut path = data_dir().expect("The operational system is not supported by this application"); 
            path.push("chameleon");
            path
        };

        let data_struct = Self {
            
            themes_path: {
                data_path.join("themes")
            },

            backup_path: {
                data_path.join("backup")
            },
            
            config_path: config_dir().expect("The operational system is not supported by this application"),
            
            data_path,

            tmp_dir: TempDir::new("chameleon").expect("Could not create temporary directory"),
        }; 
        data_struct.start();
        
        data_struct
    }

    fn start(&self) {
        self.initialize_directory();

    }

    fn initialize_directory(&self) {
        self.check_and_create_dir(&self.data_path);

        self.check_and_create_dir(&self.themes_path);
        self.check_and_create_dir(&self.backup_path);
        
    }

    fn check_and_create_dir(&self ,path: &PathBuf) {
        if !(path.exists()) {
            fs::create_dir_all(path).expect("Could not create directory, check write access on local share and config directories");
        }
    }
}


