use crate::*;
pub mod anime;
pub mod config;
pub mod search;
pub mod seasonal;
pub mod state;

pub use state::*;

// pub struct MainLayout {
//     pub menu: Vec<Rect>,
//     pub settings: Vec<Rect>,
// }

// impl MainLayout {
//     pub fn from<B: Backend>(f: &mut Frame<B>, shrink: bool) -> MainLayout {
//         let menu: Vec<Rect> = get_anime_layout(f);
//
//         let mut container_size: Vec<f64> = vec![];
//         if shrink {
//             container_size.append(&mut vec![1.0]);
//         } else {
//             container_size.append(&mut vec![2.0]);
//         }
//         container_size.append(&mut vec![2.0]);
//         container_size.append(&mut vec![1.0]);
//         container_size.append(&mut vec![1.0]);
//
//         let settings: Vec<Rect> = get_config_layout(f, container_size);
//
//         MainLayout { menu, settings }
//     }
// }

pub fn ui<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState) {
    let input_map = &app_state.input_map;
    let container: Vec<Rect> = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    // RENDER TOP BAR
    // ==============
    let tabs_vec = vec![
        ("Anime [1]", '1'),
        ("Search [2]", '2'),
        ("Updates [3]", '3'),
        ("Config [4]", '4'),
    ];

    let tabs = tabs_vec
        .iter()
        .cloned()
        .map(|t| {
            let text: Vec<String> = separator(t.0, t.1);
            // dbg!(&text);
            let start: String;
            let c: String;
            let rest: String;
            //
            //TODO: FIX MUTLIPLE LETTERS APPERING FOR THE TAB NAME
            //
            if text.len() == 2 {
                start = "".to_owned();
                c = text[0].to_owned();
                rest = text[1].to_owned();
            } else {
                start = text[0].to_owned();
                c = text[1].to_owned();
                rest = text[2].to_owned();
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

    let current_tab = (input_map.tab - input_map.tab.min(1)).min(tabs_vec.len() as u8 - 1); // dont mess with this math

    let tabs_widget = Tabs::new(tabs)
        .select(current_tab as usize)
        .highlight_style(Style::default().fg(Color::Blue))
        .block(Block::default().borders(Borders::ALL).title("Tabs"));
    f.render_widget(tabs_widget, container[0]);

    // RENDER CONTENTS
    // ===============
    match current_tab {
        0 => {
            anime::render_panel(f, container[1], &mut app_state.anime); // TODO: move f.render out of these functions and make it like below:
            // f.render_widget(main_widget, container[1]);
        }
        1 => {
            // Render content for Tab 2
        }
        2 => {
            // Render content for Tab 3
        }
        3 => config::render_panel(f, container[1]),
        
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
