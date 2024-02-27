mod data;
mod args;

use clap::Parser;
use futures::executor::block_on;

use data::DataManager;

use args::Action;

#[tokio::main]
async fn main() {
    let data_manager = DataManager::new();
    let action = args::Args::parse().get_action();

    match action {
        Action::Standby => (),
        Action::SetTheme(theme) => {
            if let Err(error) = data_manager.set_theme(&theme) {println!("{}", error)};
        },

        Action::WebInstall(url) => {
            block_on(data_manager.web_install(&url));
        }
    }
}
