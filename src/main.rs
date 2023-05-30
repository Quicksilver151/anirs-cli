//START

//stds:


//crates:
pub use tui::*;
pub use crossterm::*;
pub use colored::*;
pub use serde::{Serialize, Deserialize};


//files:
mod api;
mod ui;
mod usr;
mod utils;
//--------
// use api::*;
// use ui::*;
// use usr::*;
// use utils::*;


fn main() {
    //TODO:
    //load_config()
    //get_input_flags()
    //overwrite some configs temporarily with input flags
    //help     -> helptext
    //(allow swithing different tabs for each:)
    //anime    (_) -> anime online browse tui (default screen)
    //config   (c) -> settings menu tui
    //seasonal (s) -> seasonal anime update tui
    //files    (f) -> browse downloaded anime tui
    //tuihelp  (h) -> tui shortcuts details
    println!(" {}","Hello, world!".blue().bold());
}
