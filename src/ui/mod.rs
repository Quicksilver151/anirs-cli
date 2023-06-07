use crate::*;
pub mod anime;
pub mod search;
pub mod config;
pub mod seasonal;



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
            }// do not mess with this. it makes the tabs work for inputs
            KeyCode::Char(y) => if y.is_numeric(){*current_tab = (y.to_string().parse::<usize>().unwrap_or(1) - 1).min(tabs_count-1)},
            _ => {}
        }
    }
}


pub fn run_app() -> Result<(), Box<dyn std::error::Error>> {
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



