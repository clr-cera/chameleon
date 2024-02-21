use std::{fs, path::PathBuf};

use dirs::{self, data_dir};

pub struct DataManager {
    data_path: PathBuf,
    themes_path: PathBuf,
}

impl DataManager {
    pub fn new() -> Self{
        let data_struct = Self {
            data_path: {
                let mut path = data_dir().expect("The operational system is not supported by this application"); 
                path.push("chameleon");
                path
            },
            themes_path: {
                let mut path = data_dir().expect("The operational system is not supported by this application"); 
                path.push("chameleon");
                path.push("themes");
                path
            }
        }; 
        data_struct.start();
        
        data_struct
    }

    fn start(&self) {
        self.initialize_directory();

    }

    fn initialize_directory(&self) {
        check_and_create(&self.data_path);

        check_and_create(&self.themes_path);

    }

}

fn check_and_create(path: &PathBuf) {
    if !(path.exists()) {
        fs::create_dir_all(path).expect("Could not create directory to store data, check if your user has write access in data directory");
    }

}

