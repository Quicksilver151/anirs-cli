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
            KeyCode::Char(y) => if y.is_numeric(){*current_tab = (y.to_string().parse::<usize>().unwrap_or(1) - 1).min(tabs_count-1)},
            _ => {}
        }
    }
}

fn separator(input: &str, separator: char) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();

    for c in input.chars() {
        if c == separator {
            if !current.is_empty() {
                result.push(current.clone());
                current.clear();
            }
            result.push(c.to_string());
        } else {
            current.push(c);
        }
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    let tabs = vec![("Anime [1]",'1'), ("Search [2]",'2'), ("Seasonal [3]",'3'), ("Config [4]",'4')];
    let mut current_tab = 0;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(f.size());

            // Render the tabs
            let tabs = tabs
                .iter()
                .cloned()
                .map(|t| {
                    let words:Vec<String> = separator(t.0, t.1);
                    // dbg!(&words);
                    let start:String;
                    let c:String;
                    let rest:String;
                    //
                    //TODO: FIX MUTLIPLE LETTERS APPERING FOR THE TAB NAME
                    //
                    if words.len() == 2 {
                        start = "".to_owned();
                        c = words[0].to_owned();
                        rest = words[1].to_owned();
                    }else{
                        start = words[0].to_owned();
                        c = words[1].to_owned();
                        rest = words[2].to_owned();
                    }
                    Spans::from(vec![
                        Span::styled(start, Style::default().fg(Color::White)),
                        Span::styled(
                            c,
                            Style::default()
                                .fg(Color::Blue)
                                .add_modifier(Modifier::UNDERLINED)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(rest, Style::default().fg(Color::White)),
                    ])
                })
                .collect();
                
            let tabs_widget = Tabs::new(tabs)
                .select(current_tab)
                .highlight_style(Style::default().fg(Color::Blue))
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
