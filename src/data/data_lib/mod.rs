use std::path::PathBuf;
use std::collections::VecDeque;
use std::fs;
use std::io::Cursor;
use std::fs::File;
use std::process::exit;

use super::DataManager;
use super::data_errors::DataError;

impl DataManager {
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
    
    pub fn backup_file(&self, file_path: &PathBuf, dir_path: &PathBuf) {
        // This function backups a file that will be substituted, also considering its base
        // directory
        Self::move_file(&self.backup_path, file_path, dir_path)
    }
    
    pub fn move_file(target_path: &PathBuf, file_path: &PathBuf, dir_path: &PathBuf) {
        // This function backups a file that will be substituted, also considering its target_path
        let initial_current_dir = std::env::current_dir().unwrap();
        Self::switch_current_dir(dir_path);
      
        let parent_name = dir_path.file_name().unwrap().to_str().unwrap(); 

        let final_file_path = target_path.join(parent_name).join(Self::make_relative(file_path));

        Self::switch_current_dir(&initial_current_dir);

        Self::ensure_parent(&final_file_path);
        //println!("{}", file_path.display()); //DEBUG
        fs::rename(&file_path, &final_file_path).expect(format!("{}",DataError::WriteAccess { path: final_file_path.into() }).as_str());
    }

    pub fn copy_file(target_path: &PathBuf, file_path: &PathBuf, dir_path: &PathBuf) {
        // This function backups a file that will be substituted, also considering its target_path
        let initial_current_dir = std::env::current_dir().unwrap();
        Self::switch_current_dir(dir_path);
      
        let parent_name = dir_path.file_name().unwrap().to_str().unwrap(); 

        let final_file_path = target_path.join(parent_name).join(Self::make_relative(file_path));

        Self::switch_current_dir(&initial_current_dir);

        Self::ensure_parent(&final_file_path);
        //println!("{}", final_file_path.display()); //DEBUG
        fs::copy(&file_path, &final_file_path).expect(format!("{}",DataError::WriteAccess { path: final_file_path.into() }).as_str());
    }

    pub fn switch_current_dir(path: &PathBuf) {
        std::env::set_current_dir(path).expect(DataError::CurrentDirChange { path: path.clone() }.to_string().as_str());

    }
    
    pub async fn download_file(path: &PathBuf, url:  &str) {
        let response = reqwest::get(url).await.expect(DataError::NoConnection { url: url.to_string() }.to_string().as_str());

        let response_bytes = response.bytes().await.expect("I am not sure how could this go wrong");
        // Wrap bytes into cursor so that they can be copied
        let mut content = Cursor::new(response_bytes);

        DataManager::ensure_parent(path);
        let mut file = File::create(path).expect(DataError::WriteAccess {path: path.clone()}.to_string().as_str());
        
        std::io::copy(&mut content, &mut file).expect(DataError::WriteAccess { path: path.clone() }.to_string().as_str());
    }
   
    // This function was copied from the zip crate example with slight modifications, TODO
    // modularize and improve this function
    pub fn extract_zip(zip_path: &PathBuf, path_to_extract: &PathBuf) {
        let zipfile = File::open(zip_path).expect(DataError::WriteAccess { path: zip_path.clone()}.to_string().as_str());
        let mut archive = match zip::ZipArchive::new(zipfile){
            Err(_) => {
                println!("File is not zip.");
                std::fs::remove_file(zip_path).expect(DataError::WriteAccess { path: zip_path.clone().into() }.to_string().as_str());
                exit(1);
            }
            Ok(value) => value,
        };
        
        std::fs::create_dir_all(&path_to_extract).expect(DataError::WriteAccess { path: path_to_extract.clone().into() }.to_string().as_str());

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let outpath = path_to_extract.join(match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            });

            if (*file.name()).ends_with('/') {
                fs::create_dir_all(&outpath).unwrap();
            } 

            else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p).unwrap();
                    }
                }
                let mut outfile = fs::File::create(&outpath).unwrap();
                std::io::copy(&mut file, &mut outfile).unwrap();
            }

            // Get and Set permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
                }
            }
        }
    }
}
