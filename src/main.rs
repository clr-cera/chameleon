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
        Action::SetTheme(theme) => {
            data_manager.reset();
            data_manager.set_theme(&theme);
        },

        Action::WebInstall(url) => {
            block_on(data_manager.web_install(&url));
        },

        Action::LocalInstall(path) => {
            data_manager.local_install(&path);
        },

        Action::Reset => {
            data_manager.reset();
        },

        Action::Standby => (),
    }
}
