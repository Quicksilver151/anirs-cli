use crate::*;

#[derive(Debug, Default)]
pub struct Anime {
    pub title: String,
    pub episodes: Vec<u16>,
}
#[derive(Debug, Default)]
pub struct SeasonsFolders {
    pub anime: Vec<Anime>,
}
#[derive(Debug, Default)]
pub struct DownloadsFolders {
    pub season: Option<Vec<SeasonsFolders>>,
    pub anime: Vec<Anime>,
}

pub fn get_anime_folder_contents() -> DownloadsFolders{
    let dirs = std::path::Path::read_dir(std::path::Path::new("/home/renderinguser/Videos/Anime/")).expect("Fetching files from ~/Videos/Anime").map(|x|x.unwrap());
    let mut dir_names = {
        let mut x = vec![];
        for entry in dirs{
            if DirEntry::path(&entry).is_dir(){
                x.append(&mut vec![entry.file_name()]);
            }
        }
        x
    };
    dir_names.sort();
    let dir_name_str :Vec<String> = dir_names.iter().map(|x| x.to_str().unwrap().to_string()).collect();
    dbg!(&dir_name_str);
    
    let mut folders = DownloadsFolders::default();
    
    dir_name_str.into_iter().for_each(|x| folders.anime.append(&mut vec![Anime{title:x, episodes:vec![]}]));
    
    folders
}

