//START

//stds:
use std::io;
use io::stdout;

//crates:
pub use crossterm::{
    event::{self, 
        Event, 
        KeyCode, 
        EnableMouseCapture, 
        DisableMouseCapture,
    },
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    execute,
};
pub use tui::{
    text::{Spans, Span},
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Tabs},
    Terminal,
};
pub use colored::*;
pub use serde::{Serialize, Deserialize};

//files:
mod api;
mod ui;
mod usr;
mod utils;
//--------
// use api::*;
use ui::*;
// use usr::*;
use utils::*;


fn main() {
    run_app();
    //TODO:
    //load_config()
    //get_input_flags()
    //overwrite some configs temporarily with input flags
    //help     -> helptext
    //(allow swithing different tabs for each:)
    //anime    (1) -> browse downloaded anime tui (default screen)
    //search   (2) -> browse anime online
    //seasonal (3) -> seasonal anime update tui
    //ranks    (4) -> ranking system?
    //config   (c) -> settings menu tui
    //tuihelp  (h) -> tui shortcuts details
    // println!(" {}","Hello, world!".blue().bold());
}



