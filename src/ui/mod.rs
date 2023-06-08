use crate::*;
pub mod anime;
pub mod config;
pub mod search;
pub mod seasonal;
pub mod state;

pub use anime::*;
pub use state::*;
pub use config::*;
pub use state::*;


pub struct MainLayout {
    pub menu: Vec<Rect>,
    pub settings: Vec<Rect>,
}

impl MainLayout {
    pub fn from<B: Backend>(f: &mut Frame<B>, shrink: bool) -> MainLayout {
        let menu: Vec<Rect> = get_anime_layout(f);

        let mut container_size: Vec<f64> = vec![];
        if shrink {
            container_size.append(&mut vec![1.0]);
        } else {
            container_size.append(&mut vec![2.0]);
        }
        container_size.append(&mut vec![2.0]);
        container_size.append(&mut vec![1.0]);
        container_size.append(&mut vec![1.0]);

        let settings: Vec<Rect> = get_config_layout(f, container_size);

        MainLayout { menu, settings }
    }
}
//
// fn handle_input(event: Event, current_tab: &mut usize, tabs_count: usize) {
//     if let Event::Key(key_event) = event {
//         match key_event.code {
//             KeyCode::Char('q') => *current_tab = tabs_count, // Exit the program on 'q' key
//             KeyCode::Tab => {
//                 // Switch to the next tab
//                 *current_tab = (*current_tab + 1) % tabs_count;
//             }
//             KeyCode::BackTab => {
//                 // Switch to the previous tab
//                 *current_tab = (*current_tab + tabs_count - 1) % tabs_count;
//             } // do not mess with this. it makes the tabs work for inputs
//             KeyCode::Char(y) => {
//                 if y.is_numeric() {
//                     *current_tab =
//                         (y.to_string().parse::<usize>().unwrap_or(1) - 1).min(tabs_count - 1)
//                 }
//             }
//             _ => {}
//         }
//     }
// }
//
// pub fn run_app() -> Result<(), Box<dyn std::error::Error>> {
//     // Create a terminal and enable raw mode
//     enable_raw_mode()?;
//     execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
//     let mut stdout = stdout();
//     let backend = CrosstermBackend::new(&mut stdout);
//     let mut terminal = Terminal::new(backend)?;
//
//     // Set up the TUI application loop
//     let tabs = vec![
//         ("Anime [1]", '1'),
//         ("Search [2]", '2'),
//         ("Seasonal [3]", '3'),
//         ("Config [4]", '4'),
//     ];
//     let mut current_tab = 0;
//     
//     loop {
//         terminal.draw(|f| {
//             let chunks = Layout::default()
//                 .direction(Direction::Vertical)
//                 .margin(1)
//                 .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
//                 .split(f.size());
//
//             // Render the tabs
//             let tabs = tabs
//                 .iter()
//                 .cloned()
//                 .map(|t| {
//                     let words: Vec<String> = separator(t.0, t.1);
//                     // dbg!(&words);
//                     let start: String;
//                     let c: String;
//                     let rest: String;
//                     //
//                     //TODO: FIX MUTLIPLE LETTERS APPERING FOR THE TAB NAME
//                     //
//                     if words.len() == 2 {
//                         start = "".to_owned();
//                         c = words[0].to_owned();
//                         rest = words[1].to_owned();
//                     } else {
//                         start = words[0].to_owned();
//                         c = words[1].to_owned();
//                         rest = words[2].to_owned();
//                     }
//                     Spans::from(vec![
//                         Span::styled(start, Style::default().fg(Color::White)),
//                         Span::styled(
//                             c,
//                             Style::default()
//                                 .fg(Color::DarkGray)
//                                 .add_modifier(Modifier::UNDERLINED)
//                                 .add_modifier(Modifier::BOLD),
//                         ),
//                         Span::styled(rest, Style::default().fg(Color::White)),
//                     ])
//                 })
//                 .collect();
//
//             let tabs_widget = Tabs::new(tabs)
//                 .select(current_tab)
//                 .highlight_style(Style::default().fg(Color::DarkGray))
//                 .block(Block::default().borders(Borders::ALL).title("Tabs"));
//             f.render_widget(tabs_widget, chunks[0]);
//
//             // Render the content of the selected tab
//             match current_tab {
//                 0 => {
//                     // Render content for Tab 1
//                 }
//                 1 => {
//                     // Render content for Tab 2
//                 }
//                 2 => {
//                     // Render content for Tab 3
//                 }
//                 _ => {}
//             }
//         })?;
//
//         // Handle events (e.g., key presses)
//         if let Event::Key(key_event) = event::read()? {
//             handle_input(Event::Key(key_event), &mut current_tab, tabs.len());
//         }
//
//         // Break the loop on 'q' key press
//         if current_tab == tabs.len() {
//             break;
//         }
//     }
//
//     // Disable raw mode and restore the terminal state
//     disable_raw_mode()?;
//     execute!(
//         terminal.backend_mut(),
//         LeaveAlternateScreen,
//         DisableMouseCapture,
//     )?;
//     Ok(())
// }
//
//

// MAIN LOGIC MANAGEMENT:
pub fn run_app<B: Backend>( terminal: &mut Terminal<B>) -> Result<(), std::io::Error> {
    // inital draw
    terminal.draw(|f| ui(f, &mut InputMap::default()))?;
    

    
    // MAIN LOOP (@input)
    loop {
        let mut input_map = match InputMap::process_input_events(){
            Ok(im) => im,
            Err(problem) => panic!("failed to get input due to {}", problem),
        };
        if input_map.quit {
            return Ok(());
        }
        if input_map.back{
            dbg!(&input_map);
        }
        
        
        
        // when any tab pressed:
        // match input_map.tab {
        //     1 => print!("one"),
        //     2 => print!("two"),
        //     _ => {}
        // };
        
        terminal.draw(|f| ui(f, &mut input_map))?;
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, input_map: &mut InputMap) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    
    // RENDER TOP BAR
    // ==============
    let tabs = vec![
        ("Anime [1]", '1'),
        ("Search [2]", '2'),
        ("Seasonal [3]", '3'),
        ("Config [4]", '4'),
    ];
    
    let tabs = tabs
        .iter()
        .cloned()
        .map(|t| {
            let words: Vec<String> = separator(t.0, t.1);
            // dbg!(&words);
            let start: String;
            let c: String;
            let rest: String;
            //
            //TODO: FIX MUTLIPLE LETTERS APPERING FOR THE TAB NAME
            //
            if words.len() == 2 {
                start = "".to_owned();
                c = words[0].to_owned();
                rest = words[1].to_owned();
            } else {
                start = words[0].to_owned();
                c = words[1].to_owned();
                rest = words[2].to_owned();
            }
            Spans::from(vec![
                Span::styled(start, Style::default().fg(Color::White)),
                Span::styled(
                    c,
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::UNDERLINED)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();
        
    let current_tab = input_map.tab - input_map.tab.min(1);
    let tabs_widget = Tabs::new(tabs)
        .select(current_tab as usize)
        .highlight_style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL).title("Tabs"));
    f.render_widget(tabs_widget, chunks[0]);
    
    
    
    // RENDER CONTENTS
    // ===============
    match current_tab {
        0 => {
            let main_widget = Paragraph::new("hello").style(Style::default().fg(Color::Cyan));
            f.render_widget(main_widget, chunks[1]);
        }
        1 => {
            // Render content for Tab 2
        }
        2 => {
            // Render content for Tab 3
        }
        _ => {}
    }
}
// MAIN UI MANAGEMENT:
// fn ui<B: Backend>(f: &mut Frame<B>, input_map: &mut InputMap) {
//     fn new_block(title: &str) -> Block {
//         let block = Block::default()
//             .title(title)
//             .borders(Borders::ALL)
//             .border_type(BorderType::Rounded);
//         block
//     }
//
//     let layouts = MainLayout::from(f, input_map.shrink);
//     let menu_block = new_block("main");
//
//     let block_1 = new_block("1");
//     let block_2 = new_block("2");
//     let block_3 = new_block("3");
//     let block_4 = new_block("4");
//
//     f.render_widget(block_1, layouts.settings[0]);
//     f.render_widget(block_2, layouts.settings[1]);
//     f.render_widget(block_3, layouts.settings[2]);
//     f.render_widget(block_4, layouts.settings[3]);
//
//     f.render_widget(menu_block, layouts.menu[0]);
// }
