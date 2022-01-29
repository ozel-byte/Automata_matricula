mod views;
use iced::{
    Sandbox, Settings
};


pub fn main() -> iced::Result {
    views::window::AutomataMatricula::run(Settings::default())
}




