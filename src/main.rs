//START

//stds:
use io::stdout;
use std::{fs::DirEntry, io};

//crates:
pub use colored::*;
pub use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
pub use directories::*;
pub use serde::{Deserialize, Serialize};
pub use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Tabs, Widget},
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
//updates  (3) -> anime update ui
//ranks    (4) -> ranking system?
//config   (c) -> settings menu tui
//tuihelp  (h) -> tui shortcuts details
// println!(" {}","Hello, world!".blue().bold());
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Proper error handling for file reading

    // data load
    let config  = usr::load_config();
    let usr_dat = usr::load_usr_data();
    // dbg!(usr_dat);
    
    let folders = get_anime_folder_contents();
    // dbg!(&folders);
    // return  Ok(());
    enable_raw_mode()?;
    execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    // app inits:
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut app_state: AppState = AppState::default();
    app_state.anime.list = folders.anime.iter().map(|x| x.title.clone()).collect();
    
    // RUNNNNNNN
    let result = run_app(&mut terminal, &mut app_state);

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

// MAIN LOGIC MANAGEMENT:
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app_state: &mut AppState) -> Result<(), std::io::Error> {
    // inital draw
    terminal.draw(|f| ui(f, app_state))?;

    app_state.tabs.index = 1;
    // MAIN LOOP (@input)
    loop {
        let input_map = match InputMap::process_input_events() {
            Ok(im) => im,
            Err(problem) => panic!("failed to get input due to {}", problem),
        };
        if input_map.quit {
            return Ok(());
        }
        if input_map.back {
            dbg!(&input_map);
        }
        if input_map.up && app_state.anime.current != 0 {
            app_state.anime.current -= 1;
        }
        if input_map.down {
            app_state.anime.current += 1;
        }
        app_state.input_map = input_map;

        // when any tab pressed:
        // match input_map.tab {
        //     1 => print!("one"),
        //     2 => print!("two"),
        //     _ => {}
        // };

        terminal.draw(|f| ui(f, app_state))?;
    }
}
