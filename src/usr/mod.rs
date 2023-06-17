use crate::*;

pub mod history;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub resolution: u16,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct History {}
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Updates {
    list: Vec<String>,
}
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct UserData {
    pub history: History,
    pub updates: Updates,
}

// .config
pub fn store_config(config: Config) {
    // let base_dirs = directories::BaseDirs::new().unwrap();
    // let conf_path = directories::BaseDirs::config_dir(&base_dirs);

    match confy::store("tsuiyomi", "config", config) {
        Ok(_) => println!("Saved config successfully!"),
        Err(reason) => println!("Err: saving failed due to: {}", reason),
    }
}
pub fn load_config() -> Config {
    let load_result: Result<Config, confy::ConfyError> = confy::load("tsuiyomi", "config");
    match load_result {
        Ok(cfg) => cfg,
        Err(reason) => {
            println!(
                "Failed to load config due to: {}\nCreating new config with defaults",
                reason
            );
            Config::default()
        }
    }
}

// .local
pub fn store_usr_data(user_data: UserData) {
    let base_dirs = directories::BaseDirs::new().unwrap();
    let mut data_path = directories::BaseDirs::data_dir(&base_dirs).to_owned();
    data_path.push("tsuiyomi");
    data_path.push("dat.toml");

    match confy::store_path(data_path, user_data) {
        Ok(_) => println!("Saved config successfully!"),
        Err(reason) => println!("Err: saving failed due to: {}", reason),
    }
}
pub fn load_usr_data() -> Option<UserData> {
    let base_dirs = directories::BaseDirs::new().unwrap();
    let mut data_path = directories::BaseDirs::data_dir(&base_dirs).to_owned();
    data_path.push("tsuiyomi");
    data_path.push("dat.toml");

    let load_result: Result<UserData, confy::ConfyError> = confy::load_path(&data_path);
    match load_result {
        Ok(usr_dat) => Some(usr_dat),
        Err(reason) => {
            println!(
                "Failed to load config due to: {}\nPlease either delete or rename the file in {}",
                reason,
                data_path.to_owned().to_str().unwrap()
            );
            None
        }
    }
}
// bad idea, need to simplify instead, no impls, no traits
