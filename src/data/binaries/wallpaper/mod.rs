use std::path::PathBuf;

pub enum WallpaperSetter {
    Swww,
}


impl WallpaperSetter {
    pub fn get_command(&self, wallpaper_path: &PathBuf) -> String{
        return match self {
            Self::Swww => format!("swww img {}", wallpaper_path.display()),
        };
        
    }
}


