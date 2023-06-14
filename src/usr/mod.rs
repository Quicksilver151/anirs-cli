pub mod history;

pub struct History{}
pub struct Updates{
    list: Vec<String>,
}

pub struct UserData {
    pub history: History,
    pub updates: Updates
}

// .config
pub fn store_config() {
    let base_dirs = directories::BaseDirs::new().unwrap();
    let conf_path = directories::BaseDirs::config_dir(&base_dirs);
    
    
    
    
}

// .local
pub fn store_data(updates: Updates){
    let base_dirs = directories::BaseDirs::new().unwrap();
    let data_path = directories::BaseDirs::data_dir(&base_dirs);
    
    
    
    
}










