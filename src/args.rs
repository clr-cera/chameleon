use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Set theme
    #[arg(short, long, default_value_t = String::new())]
    pub set: String,
}

impl Args {
    pub fn get_action(&self) -> Action{
        if self.set.len() > 0 {Action::SetTheme(self.set.clone())}
        else {Action::Standby}
    }
}

pub enum Action {
    SetTheme(String),
    Standby,
}
