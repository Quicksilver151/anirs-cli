use crate::*;

pub fn render_panel<B: Backend>(f: &mut Frame<B>, area: Rect){
    
    let widget = Paragraph::new("hello").style(Style::default().fg(Color::Cyan));
    f.render_widget(widget, area)
    
}

