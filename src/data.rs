use std::collections::HashMap;
use std::io::BufReader;
use serde::Deserialize;

#[derive(Deserialize)]
struct Database {
    data: Vec<RawEntry>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawEntry {
    title: String,
    #[serde(rename = "type")]
    kind: String,
    episodes: Option<u32>,
    anime_season: Option<Season>,
    tags: Option<Vec<String>>,
    sources: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Season {
    season: Option<String>,
    year: Option<u32>,
}

pub struct Anime {
    pub mal_id: u32,
    pub title: String,
    pub kind: String,
    pub genres: Vec<String>,
    pub episodes: Option<u32>,
    pub year: Option<u32>,
    pub season: Option<String>,
}

fn mal_id_from_sources(sources: &[String]) -> Option<u32> {
    for url in sources {
        if url.contains("myanimelist.net/anime/") {
            let id = url.rsplit("/").next()?;
            return id.parse().ok();
        }
    }
    None
}

pub fn catalog(path: &str) -> Vec<Anime> {
    let file = std::fs::File::open(path).expect("could not open database file");
    let db: Database =
        serde_json::from_reader(BufReader::new(file)).expect("Could not parse database JSON");

    db.data
        .into_iter()
        .filter_map(|e| {
            let mal_id = mal_id_from_sources(&e.sources)?;
            let title_trim = e.title.trim();

            if e.kind == "SPECIAL" || e.kind == "MUSIC" || e.kind == "CM" {
                return None;
            }

            let first_char = title_trim.chars().next()?;
            if !first_char.is_alphanumeric() {
                return None;
            }

            Some(Anime {
                mal_id,
                title: e.title,
                kind: e.kind,
                genres: e.tags.unwrap_or_default(),
                episodes: e.episodes,
                year: e.anime_season.as_ref().and_then(|s| s.year),
                season: e.anime_season.as_ref().and_then(|s| s.season.clone()),
            })
        })
        .collect()
}

pub fn by_genre(catalog: &[Anime]) -> HashMap<String, Vec<&Anime>> {
    let mut map: HashMap<String, Vec<&Anime>> = HashMap::new();
    for anime in catalog {
        for genre in &anime.genres {
            map.entry(genre.clone()).or_default().push(anime);
        }
    }
    map
}

pub fn by_kind<'a>(catalog: &'a [Anime], kind: &str) -> Vec<&'a Anime> {
    catalog.iter().filter(|a| a.kind == kind).collect()
}