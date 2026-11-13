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
 
            shared.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap());
            Some(Rec {
                anime: a,
                score,
                shared: shared.into_iter().take(4).map(|(g, _)| g).collect(),
            })
        })
        .collect();
 
    recs.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    recs.truncate(limit);
    recs
}
 
fn pick_favorites(catalog: &[Anime]) -> Option<Vec<Favorite>> {
    let mut chosen: Vec<Favorite> = Vec::new();
 
    while chosen.len() < PICKS {
        let options: Vec<String> = catalog
            .iter()
            .filter(|a| !chosen.iter().any(|c| c.mal_id == a.mal_id))
            .map(|a| a.title.clone())
            .collect();
 
        let prompt = format!("Pick anime {}/{} (type to search):", chosen.len() + 1, PICKS);
        let title = Select::new(&prompt, options).with_page_size(10).prompt().ok()?;
        let anime = catalog.iter().find(|a| a.title == title)?;
        chosen.push(Favorite {
            mal_id: anime.mal_id,
            title: anime.title.clone(),
        });
    }
    Some(chosen)
}
 
pub fn run(catalog: &[Anime]) {
    if catalog.is_empty() {
        println!("No anime found in database!");
        return;
    }
 
    let mut profile = load_profile();
 
    let reuse = if profile.favorites.len() == PICKS {
        println!("\nYour saved picks:");
        for f in &profile.favorites {
            println!("  - {}", f.title);
        }
        let choice = Select::new("", vec!["Use these", "Pick new ones"]).prompt();
        matches!(choice, Ok("Use these"))
    } else {
        false
    };
 
    if !reuse {
        match pick_favorites(catalog) {
            Some(favs) => {
                profile.favorites = favs;
                save_profile(&profile);
            }
            None => {
                println!("Cancelled.");
                return;
            }
        }
    }
 
    let favs: Vec<&Anime> = profile
        .favorites
        .iter()
        .filter_map(|f| catalog.iter().find(|a| a.mal_id == f.mal_id))
        .collect();
 
    let recs = recommend(catalog, &favs, RESULTS);
    if recs.is_empty() {
        println!("\nNot enough data to recommend anything from those picks.");
        return;
    }
 
    println!("\nBased on your picks, you might like:\n");
    for (i, r) in recs.iter().enumerate() {
        let year = r.anime.year.map(|y| y.to_string()).unwrap_or_else(|| "?".into());
        println!("{:>2}. {} ({}, {})", i + 1, r.anime.title, r.anime.kind, year);
        println!("    because: {}", r.shared.join(", "));
    }
}
 