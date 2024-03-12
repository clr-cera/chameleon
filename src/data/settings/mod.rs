use std::path::PathBuf;
use std::{env, fs};
use std::process::Command;

pub struct BinManager {
}

enum WallpaperSetter {
    Swww,
}

impl WallpaperSetter {
    pub fn set(&self, wallpaper_path: &PathBuf) {
        let command_text: String = match self {
            Self::Swww => format!("swww img {}", wallpaper_path.display()),
        };
        
        let mut command = Command::new("sh"); command.arg("-c");
        command.arg(command_text.as_str());
        
        command.spawn().expect(format!("Could not execute command {}", command_text).as_str());
    }
}
impl BinManager {
    pub fn set_wallpaper(wallpaper_path: &PathBuf) {
        let setter = if let Some(value) = Self::get_wallpaper_setter() {
            value
        } else {return};

        setter.set(wallpaper_path);
    }

    fn get_wallpaper_setter() -> Option<WallpaperSetter> {
        if Self::is_program_in_path("swww") {return Some(WallpaperSetter::Swww)}

        None
    }
    
    fn is_program_in_path(program: &str) -> bool {
        if let Ok(path) = env::var("PATH") {
            for p in path.split(":") {
                let p_str = format!("{}/{}", p, program);
                if fs::metadata(p_str).is_ok() {
                    return true;
                }
            }
        }
        false
    }
}
