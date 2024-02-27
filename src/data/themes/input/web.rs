use crate::data::data_errors::DataError;
use crate::DataManager;

impl DataManager {
    pub async fn web_install(&self, url: &str) { 
        let zip_path = self.themes_path.join("download.zip");
        
        Self::download_file(&zip_path, url).await;

        let theme_name = String::from("New");
        
        let _new_theme_path = Self::extract_zip(&zip_path, &theme_name);
        std::fs::remove_file(&zip_path).expect(DataError::WriteAccess { path: zip_path.clone() }.to_string().as_str());
    }


}
