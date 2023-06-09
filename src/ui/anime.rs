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
    
    let anime_list : Vec<ListItem> = vec![

        ListItem::new("hello"),
        ListItem::new("hello1"),
        ListItem::new("hell2"),
        ListItem::new("hellr").style(Style::default().bg(Color::Red)),
        ListItem::new("hell4"),
        ListItem::new("hell5"),
        ListItem::new("hell5"),
        ListItem::new("hell5"),
        ListItem::new("hell5"),
        ListItem::new("hell5"),
        ListItem::new("hell5"),
        ListItem::new("hell5"),
        ListItem::new("hell5"),
        ListItem::new("hell5"),
        ListItem::new("hell5"),
        ListItem::new("hell5"),
    ];
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

