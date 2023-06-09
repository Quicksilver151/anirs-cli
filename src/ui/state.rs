use crate::*;

#[derive(Debug, Default)]
pub struct Tab;

#[derive(Debug, Default)]
pub struct State {
    pub tabs: Tab,
    pub input: char,
    pub input_map: InputMap,
}
impl State {
}

#[derive(Debug, Default)]
pub struct InputMap {
    pub rename: bool,
    pub quit: bool,
    pub shrink: bool,

    pub next: bool,
    pub prev: bool,

    pub enter: bool,
    pub back: bool,

    pub up: bool,
    pub down: bool,

    pub tab: u8,
    pub charachter: char,

}
impl InputMap {
    // Mutates the value
    pub fn process_input_events() -> Result<InputMap, std::io::Error>{
        let mut input_map = InputMap::default();
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => input_map.quit = true,
                KeyCode::Char('r') => input_map.rename = true,
                
                KeyCode::Char(x) => {
                    if x.is_numeric() {
                        input_map.tab = x.to_digit(10).unwrap() as u8;
                    }else{
                        input_map.charachter = x;
                    }
                },
                
                KeyCode::Right | KeyCode::Enter => input_map.enter = true,
                KeyCode::Left | KeyCode::Esc => input_map.back = true,
                
                KeyCode::Up | KeyCode::BackTab => input_map.up = true,
                KeyCode::Down | KeyCode::Tab => input_map.down = true,
                
                _ => {}
            }
        }
        
        Ok(input_map)
        
    }
}


