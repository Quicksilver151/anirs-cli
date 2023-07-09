use crate::*;

#[derive(Debug, Default, Clone)]
pub struct Anime {
    pub title: String,
    pub desc: String,
    pub episodes: Vec<u16>,
}
#[derive(Debug, Default)]
pub struct SeasonsFolders {
    pub anime: Vec<Anime>,
}
#[derive(Debug, Default)]
pub struct DownloadsFolders {
    pub season: Option<Vec<SeasonsFolders>>,
    pub anime_list: Vec<Anime>,
}

pub fn read_desc(path: &mut std::path::PathBuf) -> String {
    path.push(".desc");
    std::fs::read_to_string(&path).unwrap_or(format!("no description found in:\n{}", path.clone().to_str().unwrap()))
}

pub fn get_anime_folder_contents() -> DownloadsFolders {
    let dirs = std::path::Path::read_dir(std::path::Path::new("/home/renderinguser/Videos/Anime/"))
        .expect("Fetching files from ~/Videos/Anime")
        .map(|x| x.unwrap());
    
    let mut folders: Vec<std::path::PathBuf> = vec![];
    for entry in dirs {
        if fs::DirEntry::path(&entry).is_dir() {
            folders.append(&mut vec![entry.path()]);
        }
    }
    folders.sort();
    
    let mut anime_list :Vec<Anime> = vec![];
    for mut folder in folders.into_iter() {
        let title = folder.file_name().unwrap().to_str().unwrap().to_string();
        let desc  = read_desc(&mut folder);
        let episodes = vec![];
        
        anime_list.append(&mut vec![Anime {title, desc, episodes}]);
    }
    
    DownloadsFolders { anime_list, ..DownloadsFolders::default() }
    
    
}
