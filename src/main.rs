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
    println!(" {}","Hello, world!".blue().bold());
}
