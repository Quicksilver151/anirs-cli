use crate::*;

#[derive(Debug, Default)]
pub struct State{
    pub current: u32,
}

pub fn render_panel<B: Backend>(f: &mut Frame<B>, area: Rect, anime_state: &State){
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([
                     Constraint::Percentage(50),
                     Constraint::Length(1),
                     Constraint::Percentage(50),
        ])
        .split(area);
    let anime_name_list :Vec<&str> = vec![
        "hello",
        "h1llo",
        "heaaallo",
        "3ello",
        "hello",
        "hello",
        "hello",
        "hello",
        "hello",
        "hello",
        "hello",
        "hello",
        "hello",
        "hello",
    ];
    let anime_list: Vec<ListItem> = anime_name_list.into_iter().enumerate().map(
        |x| {
            if x.0 == anime_state.current as usize {
                ListItem::new(x.1).style(Style::default().bg(Color::Red))
            }else {
                ListItem::new(x.1)
            }
        }
    ).collect();
   
    // let widget = Paragraph::new("hello").style(Style::default().fg(Color::Cyan));
    let list = Block::default().title("list").borders(Borders::ALL);
    let actual_list = List::new(anime_list);
    let desc = Block::default().title("desc").borders(Borders::ALL);
    
    f.render_widget(list, chunks[0]);
    let margin_container = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .margin(1)
        .split(chunks[0]);
    f.render_widget(actual_list, margin_container[0]);
    f.render_widget(desc, chunks[2]);
}

