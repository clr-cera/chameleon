use std::path::PathBuf;
use std::{env, fs};
use std::process::Command;

mod wallpaper;
use wallpaper::WallpaperSetter;


pub struct BinManager {
}

impl BinManager {
    pub fn set_wallpaper(wallpaper_path: &PathBuf) {
        let setter = if let Some(value) = Self::get_wallpaper_setter() {
            value
        } else {return};

        let command_text = setter.get_command(wallpaper_path);
        Self::execute(&command_text);
    }

    // TODO throw this into WallpaperSetter itself
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

    fn execute(command_text: &str) {
        let mut command = Command::new("sh"); command.arg("-c");
        command.arg(command_text);
        
        command.spawn().expect(format!("Could not execute command {}", command_text).as_str());
    }
}
