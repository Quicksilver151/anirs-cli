use crate::*;

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
    
    let x = Config::default();
    x.save()
    
    
}

// .local
pub fn store_data(updates: Updates){
    let base_dirs = directories::BaseDirs::new().unwrap();
    let data_path = directories::BaseDirs::data_dir(&base_dirs);
    
    
    
    
}





#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub resolution: u16,
}
impl Store for Config{}
pub trait Store {
    
    fn load() -> Config{
        let load_result: Result<Config, confy::ConfyError> = confy::load("tsuiyomi", "config");
        match load_result {
            Ok (cfg) => cfg,
            Err(reason) => {
                println!("Failed to load config due to: {}\nCreating new config with defaults", reason);
                // confy::store("quran-ref", "conf", cfg)
                Config::default()
            }
        }
    }
    
    fn save(&self) where Self: Serialize{
        match confy::store("tsuiyomi", "config", self) {
            Ok(_) => println!("Saved config successfully!"),
            Err(reason) => println!("Err: saving failed due to: {}", reason),
        }
    }
    
}












