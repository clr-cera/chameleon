mod data;
mod args;


use clap::Parser;
use data::DataManager;

use args::Action;

fn main() {
    let data_manager = DataManager::new();
    let action = args::Args::parse().get_action();

    match action {
        Action::Standby => (),
        Action::SetTheme(theme) => {
            if let Err(error) = data_manager.set_theme(&theme) {println!("{}", error)};
        },
    }
}
