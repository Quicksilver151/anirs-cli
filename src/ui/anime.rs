use crate::*;


pub fn render_panel<B: Backend>(f: &mut Frame<B>, area: Rect){
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([
                     Constraint::Percentage(50),
                     Constraint::Length(1),
                     Constraint::Percentage(50),
        ])
        .split(area);
    
    // let widget = Paragraph::new("hello").style(Style::default().fg(Color::Cyan));
    let list = Block::default().title("list").borders(Borders::ALL);
    let desc = Block::default().title("desc").borders(Borders::ALL);

    f.render_widget(list, chunks[0]);
    f.render_widget(desc, chunks[2]);
}

