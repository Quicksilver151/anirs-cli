//START

//stds:
use std::io;
use io::{stdout, Write};

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
};
pub use tui::{
    text::Spans,
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
// use ui::*;
// use usr::*;
// use utils::*;


// fn main() {
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
    // println!(" {}","Hello, world!".blue().bold());
// }




fn handle_input(event: Event, current_tab: &mut usize, tabs_count: usize) {
    if let Event::Key(key_event) = event{
        match key_event.code {
            KeyCode::Char('q') => *current_tab = tabs_count, // Exit the program on 'q' key
            KeyCode::Tab => {
                // Switch to the next tab
                *current_tab = (*current_tab + 1) % tabs_count;
            }
            KeyCode::BackTab => {
                // Switch to the previous tab
                *current_tab = (*current_tab + tabs_count - 1) % tabs_count;
            }
            KeyCode::Char(y) => {
                *current_tab = match y {
                    'a' => 0usize,
                    's' => 1usize,
                    'f' => 2usize,
                    _ => *current_tab,
                }
            },
            // Add your own custom key mappings here
            _ => {}
        }
    }
}

fn main() -> Result<()> {
    // Create a terminal and enable raw mode
    enable_raw_mode()?;
    execute!(
        std::io::stdout(),
        EnterAlternateScreen,
        EnableMouseCapture
    )?;
    let mut stdout = stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    // Set up the TUI application loop
    let mut tabs = vec!["Anime Search", "Seasonal", "Files"];
    let mut current_tab = 0;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(f.size());

            // Render the tabs
            let tabs_widget = Tabs::new(tabs.iter().cloned().map(Spans::from).collect())
                .select(current_tab)
                .highlight_style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Tabs"));
            f.render_widget(tabs_widget, chunks[0]);

            // Render the content of the selected tab
            match current_tab {
                0 => {
                    // Render content for Tab 1
                }
                1 => {
                    // Render content for Tab 2
                }
                2 => {
                    // Render content for Tab 3
                }
                _ => {}
            }
        })?;

        // Handle events (e.g., key presses)
        if let Event::Key(key_event) = event::read()? {
            handle_input(Event::Key(key_event), &mut current_tab, tabs.len());
        }

        // Break the loop on 'q' key press
        if current_tab == tabs.len() {
            break;
        }
    }


    // Disable raw mode and restore the terminal state
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    Ok(())
}
