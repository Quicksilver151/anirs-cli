//START

//stds:
use io::stdout;
use std::io;

//crates:
pub use colored::*;
pub use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
pub use serde::{Deserialize, Serialize};
pub use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Tabs},
    Frame, Terminal,
};

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

// fn main() {
    // run_appp();
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
// }



fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    execute!(
        std::io::stdout(),
        EnterAlternateScreen,
        EnableMouseCapture
    )?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;
    
    let result = run_app(&mut terminal);
    
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    if let Err(e) = result {
        println!("{}", e);
    }
    
    
    Ok(())
}
