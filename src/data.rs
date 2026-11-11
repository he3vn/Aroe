use std::collections::HashMap;
use std::io::BufReader;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

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
    pub anilist_id: Option<u32>,
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

fn anilist_id_from_sources(sources: &[String]) -> Option<u32> {
    for url in sources {
        if url.contains("anilist.co/anime/") {
            let id = url.rsplit('/').next()?;
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
            let anilist_id = anilist_id_from_sources(&e.sources);
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
                anilist_id,
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

// ---------- AniList: synopsis + where to watch ----------

const ANILIST_URL: &str = "https://graphql.anilist.co";
const MEDIA_FIELDS: &str =
    "title { romaji english } description(asHtml: false) externalLinks { site url type }";

const CACHE_PATH: &str = "cache/details.json";
const CACHE_MAX_AGE_SECS: u64 = 60 * 60 * 24 * 30; // 30 days

#[derive(Serialize, Deserialize, Clone)]
pub struct Details {
    pub synopsis: String,
    pub streams: Vec<(String, String)>, // (site, url)
    pub fetched_at: u64,
}

#[derive(Deserialize)]
struct AniListResponse {
    data: Option<AniListData>,
}

#[derive(Deserialize)]
struct AniListData {
    #[serde(rename = "Media")]
    media: Option<AniListMedia>,
}

#[derive(Deserialize)]
struct AniListMedia {
    title: AniListTitle,
    description: Option<String>,
    #[serde(rename = "externalLinks")]
    external_links: Option<Vec<AniListLink>>,
}

#[derive(Deserialize)]
struct AniListTitle {
    romaji: Option<String>,
    english: Option<String>,
}

#[derive(Deserialize)]
struct AniListLink {
    site: String,
    url: String,
    #[serde(rename = "type")]
    kind: Option<String>,
}

fn post_graphql(body: serde_json::Value) -> Option<AniListMedia> {
    let response: AniListResponse = reqwest::blocking::Client::builder()
        .user_agent("aroe/0.1")
        .build()
        .ok()?
        .post(ANILIST_URL)
        .json(&body)
        .send()
        .ok()?
        .json()
        .ok()?;

    response.data?.media
}

fn normalize_title(title: &str) -> String {
    title
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

fn titles_match(expected: &str, media: &AniListMedia) -> bool {
    let expected = normalize_title(expected);

    [media.title.romaji.as_deref(), media.title.english.as_deref()]
        .into_iter()
        .flatten()
        .any(|title| normalize_title(title) == expected)
}

fn clean_description(raw: &str) -> String {
    let raw = raw
        .replace("<br>", "\n")
        .replace("<br/>", "\n")
        .replace("<br />", "\n");

    let mut out = String::new();
    let mut in_tag = false;

    for c in raw.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }

    out.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter(|line| {
            let lower = line.to_lowercase();
            !lower.starts_with("note:")
                && !lower.starts_with("(source")
                && !lower.starts_with("source:")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn load_cache() -> HashMap<String, Details> {
    std::fs::read_to_string(CACHE_PATH)
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default()
}

fn save_cache(cache: &HashMap<String, Details>) {
    if let Some(dir) = std::path::Path::new(CACHE_PATH).parent() {
        let _ = std::fs::create_dir_all(dir);
    }

    if let Ok(text) = serde_json::to_string_pretty(cache) {
        let _ = std::fs::write(CACHE_PATH, text);
    }
}

fn fetch_details(anime: &Anime) -> Option<Details> {
    let mut media = None;

    // Try the exact AniList id first (avoids duplicate-MAL-id mistakes).
    if let Some(id) = anime.anilist_id {
        media = post_graphql(serde_json::json!({
            "query": format!(
                "query($id:Int){{ Media(id:$id) {{ {} }} }}",
                MEDIA_FIELDS
            ),
            "variables": { "id": id }
        }));
    }

    // Fall back to MAL id.
    if media.is_none() {
        media = post_graphql(serde_json::json!({
            "query": format!(
                "query($idMal:Int){{ Media(idMal:$idMal) {{ {} }} }}",
                MEDIA_FIELDS
            ),
            "variables": { "idMal": anime.mal_id }
        }));
    }

    // If the title doesn't match, search by title instead.
    let needs_search = media
        .as_ref()
        .map_or(true, |m| !titles_match(&anime.title, m));

    if needs_search {
        if let Some(found) = post_graphql(serde_json::json!({
            "query": format!(
                "query($search:String){{ Media(search:$search, type:ANIME) {{ {} }} }}",
                MEDIA_FIELDS
            ),
            "variables": { "search": anime.title }
        })) {
            media = Some(found);
        }
    }

    let media = media?;

    let synopsis = clean_description(media.description.as_deref().unwrap_or(""));

    let streams = media
        .external_links
        .unwrap_or_default()
        .into_iter()
        .filter(|link| link.kind.as_deref() == Some("STREAMING"))
        .map(|link| (link.site, link.url))
        .collect();

    Some(Details {
        synopsis,
        streams,
        fetched_at: now(),
    })
}

pub fn details(anime: &Anime) -> Option<Details> {
    let key = anime.mal_id.to_string();
    let mut cache = load_cache();

    if let Some(cached) = cache.get(&key) {
        if now().saturating_sub(cached.fetched_at) < CACHE_MAX_AGE_SECS {
            return Some(cached.clone());
        }
    }

    let fresh = fetch_details(anime)?;
    cache.insert(key, fresh.clone());
    save_cache(&cache);

    Some(fresh)
}