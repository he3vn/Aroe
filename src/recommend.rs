use crate::data::Anime;
use inquire::Select;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
 
const PROFILE_PATH: &str = "data/profile.json";
const PICKS: usize = 3;
const RESULTS: usize = 10;
 
#[derive(Serialize, Deserialize, Clone)]
pub struct Favorite {
    pub mal_id: u32,
    pub title: String,
}
 
#[derive(Serialize, Deserialize, Default)]
pub struct Profile {
    pub favorites: Vec<Favorite>,
}
 
fn load_profile() -> Profile {
    std::fs::read_to_string(PROFILE_PATH)
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or_default()
}
 
fn save_profile(profile: &Profile) {
    if let Some(dir) = std::path::Path::new(PROFILE_PATH).parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    if let Ok(text) = serde_json::to_string_pretty(profile) {
        let _ = std::fs::write(PROFILE_PATH, text);
    }
}
 
fn norm(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}
 
pub struct Rec<'a> {
    pub anime: &'a Anime,
    pub score: f64,
    pub shared: Vec<String>,
}
 
/// Content-based recommender:
/// - every tag gets an IDF weight (rare tags like "time travel" count more than "comedy")
/// - the taste profile = sum of the IDF weights of the tags in your picks
/// - each candidate is scored on the weighted tags it shares, normalised by its
///   own tag count, then nudged for same type and for a similar year
pub fn recommend<'a>(catalog: &'a [Anime], favs: &[&Anime], limit: usize) -> Vec<Rec<'a>> {
    let n = catalog.len() as f64;
 
    let mut df: HashMap<&str, usize> = HashMap::new();
    for a in catalog {
        for g in &a.genres {
            *df.entry(g.as_str()).or_default() += 1;
        }
    }
    let idf = |tag: &str| -> f64 {
        let d = *df.get(tag).unwrap_or(&0) as f64;
        (n / (1.0 + d)).ln().max(0.0)
    };
 
    let mut taste: HashMap<&str, f64> = HashMap::new();
    for f in favs {
        for g in &f.genres {
            *taste.entry(g.as_str()).or_default() += idf(g);
        }
    }
 
    let years: Vec<u32> = favs.iter().filter_map(|f| f.year).collect();
    let avg_year = if years.is_empty() {
        None
    } else {
        Some(years.iter().sum::<u32>() as f64 / years.len() as f64)
    };
    let fav_kinds: HashSet<&str> = favs.iter().map(|f| f.kind.as_str()).collect();
    let fav_ids: HashSet<u32> = favs.iter().map(|f| f.mal_id).collect();
    let fav_titles: Vec<String> = favs.iter().map(|f| norm(&f.title)).collect();
 
    let mut recs: Vec<Rec> = catalog
        .iter()
        .filter(|a| !fav_ids.contains(&a.mal_id))
        .filter_map(|a| {
            // skip sequels / seasons of something you already picked
            let t = norm(&a.title);
            if fav_titles.iter().any(|ft| t.starts_with(ft) || ft.starts_with(&t)) {
                return None;
            }
 
            let mut sum = 0.0;
            let mut shared = Vec::new();
            for g in &a.genres {
                if let Some(w) = taste.get(g.as_str()) {
                    sum += w;
                    shared.push((g.clone(), *w));
                }
            }
            if shared.len() < 2 {
                return None;
            }
 
            let mut score = sum / (a.genres.len() as f64).sqrt();
            if fav_kinds.contains(a.kind.as_str()) {
                score *= 1.1;
            }
            if let (Some(avg), Some(y)) = (avg_year, a.year) {
                let closeness = (1.0 - (y as f64 - avg).abs() / 20.0).max(0.0);
                score *= 1.0 + 0.15 * closeness;
            }