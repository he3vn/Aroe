use std ::collections:: HashMap;
use serde::Deserialize;

// This part is going to be me parsing/reading the json i need it in categories so i can randomize the anime being given and make sure when one is given it cant be anymore

#[derive(Deserialize)]
struct Database {
    data: Vec<RawEntry>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawEntry {
    title: String,
    #[serde(rename="type")]
    kind: String,
    episodes: Option<u32>,
    anime_season:Option<Season>,
    tags: Option<Vec<String>>,
    sources: Vec<String>,
}

#[derive(Deserialize)]
pub struct Anime {
    pub mal_id: u32,
    pub title: String,
    pub kind: String,
    pub genres: Vec<String>, // lowercase, e.g. "action", "romance"
    pub episodes: Option<u32>,
    pub year: Option<u32>,
    pub season: Option<String>,
}

fn mal_id_from_sources(sources: &[String]) -> Option<u32> {
    for url in sources {
        if url.contains("myanimelist.net/anime/") {
            let id= url.rsplit("/").next()?;
            return id.parse().ok();
        }
    }
    None 
}

