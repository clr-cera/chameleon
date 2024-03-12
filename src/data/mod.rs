use std::{fs, path::PathBuf};

use dirs::{config_dir, data_dir, home_dir};

mod themes;
mod data_errors;
mod data_lib;
mod settings;

use tempdir::TempDir;
use data_errors::DataError;


pub struct DataManager {
    data_path: PathBuf,

    themes_path: PathBuf,
    backup_path: PathBuf,
    
    config_path:PathBuf,
    scripts_path: PathBuf,

    tmp_dir: TempDir,
}

impl DataManager {
    pub fn new() -> Self{
        let data_path = {
            let mut path = data_dir().expect(DataError::OSNotSupported.to_string().as_str());
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
            
            data_path,
            
            config_path: config_dir().expect(DataError::OSNotSupported.to_string().as_str()),
            scripts_path: home_dir().expect(DataError::OSNotSupported.to_string().as_str()).join(".scripts"),

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


