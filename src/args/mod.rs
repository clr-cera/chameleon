use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Set theme
    #[arg(short, long, default_value_t = String::new())]
    pub set: String,

    #[arg(short, long, default_value_t = String::new())]
    pub web_install: String,
    
    #[arg(short, long, default_value_t = String::new())]
    pub local_install: String,

    #[arg(short, long, default_value_t = false)]
    pub reset: bool
}

impl Args {
    pub fn get_action(&self) -> Action{
        if self.reset == true {Action::Reset}

        else if self.set.len() > 0 {Action::SetTheme(self.set.clone())}

        else if self.web_install.len() > 0 {Action::WebInstall(self.web_install.clone())}

        else if self.local_install.len() > 0 {Action::LocalInstall(self.local_install.clone())}

        else {Action::Standby}
    }
}

pub enum Action {
    SetTheme(String),
    WebInstall(String),
    LocalInstall(String),
    Standby,
    Reset,
}
