use crate::*;

pub fn get_config_layout<B: Backend>(f: &mut Frame<B>, container_size: Vec<f64>) -> Vec<Rect> {

    
    let total = container_size[0]/100.0 + container_size[1]/100.0 + container_size[2]/100.0 + container_size[3]/100.0;
    
    let containers : Vec<u16> = container_size.iter().map(|x| (x/total) as u16 ).collect::<Vec<u16>>();
    
    
    // dbg!(total);
    // dbg!(&containers);
    
    Layout::default()
        .direction(Direction::Horizontal) .constraints([
            Constraint::Percentage(containers[0]),
            Constraint::Percentage(containers[1]),
            Constraint::Percentage(containers[2]),
            Constraint::Percentage(containers[3]),
        ].as_ref()
    ).split(f.size())
}

pub fn render_panel<B: Backend>(f: &mut Frame<B>, area: Rect){
    
    let widget = Paragraph::new("helo,,,, we do configs here....").style(Style::default().fg(Color::Cyan));
    f.render_widget(widget, area)
    
}
