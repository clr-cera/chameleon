use std::thread::sleep;
use std::time::Duration;

use super::Theme;
use super::super::binaries::BinManager;
impl Theme {
    pub fn fixup(&self) {
        for command in &self.data.fixup_inline {
            BinManager::execute(&command);
            sleep(Duration::from_millis(20));
        }

        self.set_wallpaper();
    }

    pub fn set_wallpaper(&self) {
        if self.data.wallpaper.len() > 0 {
            BinManager::set_wallpaper(&self.join_path(&self.data.wallpaper))
        }
        
    }
}
