//  its basically the catalog parsing, search, curated vibes and the recommender.
// also the  catalog is the bundled manami anime database (`data/data.json`).

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Anime {
    pub mal_id: u32,
    pub anilist_id: Option<u32>,
    pub title: String,
    pub kind: String,
    pub genres: Vec<String>,
    pub episodes: Option<u32>,
    pub year: Option<u32>,
    pub season: Option<String>,

    pub score: Option<f32>,
}

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
    score: Option<RawScore>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawScore {
    arithmetic_mean: Option<f32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Season {
    season: Option<String>,
    year: Option<u32>,
}

fn id_from(sources: &[String], needle: &str) -> Option<u32> {
    sources
        .iter()
        .find(|u| u.contains(needle))
        .and_then(|u| u.rsplit('/').next())
        .and_then(|id| id.parse().ok())
}

/// Read and slim the bundled database into the in-memory catalog.
pub fn load(path: &Path) -> anyhow::Result<Vec<Anime>> {
    let file = std::fs::File::open(path)
        .map_err(|e| anyhow::anyhow!("open {}: {e}", path.display()))?;
    let db: Database = serde_json::from_reader(BufReader::new(file))
        .map_err(|e| anyhow::anyhow!("parse {}: {e}", path.display()))?;

    let mut out = Vec::with_capacity(db.data.len());
    for entry in db.data {
        let Some(mal_id) = id_from(&entry.sources, "myanimelist.net/anime/") else {
            continue;
        };
        if matches!(entry.kind.as_str(), "SPECIAL" | "MUSIC" | "CM") {
            continue;
        }
        let title = entry.title.trim().to_string();
        let Some(first) = title.chars().next() else {
            continue;
        };
        if !first.is_alphanumeric() {
            continue;
        }
        out.push(Anime {
            mal_id,
            anilist_id: id_from(&entry.sources, "anilist.co/anime/"),
            title,
            kind: entry.kind,
            genres: entry.tags.unwrap_or_default(),
            episodes: entry.episodes,
            year: entry.anime_season.as_ref().and_then(|s| s.year),
            season: entry.anime_season.as_ref().and_then(|s| s.season.clone()),
            score: entry.score.and_then(|s| s.arithmetic_mean),
        });
    }
    Ok(out)
}

/// Look for the catalog: `$AROE_DATA`, then `./data/data.json`, then next to
/// the binary, then the per-user data dir used by installed builds.
pub fn default_catalog_path() -> PathBuf {
    if let Ok(p) = std::env::var("AROE_DATA") {
        return PathBuf::from(p);
    }
    let mut candidates = vec![PathBuf::from("data/data.json")];
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.join("data/data.json"));
            candidates.push(dir.join("../../../data/data.json"));
        }
    }
    candidates.push(data_dir().join("data.json"));
    candidates
        .iter()
        .find(|p| p.exists())
        .cloned()
        .unwrap_or_else(|| PathBuf::from("data/data.json"))
}

fn data_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("XDG_DATA_HOME") {
        if !dir.is_empty() {
            return PathBuf::from(dir).join("aroe");
        }
    }
    match std::env::var("HOME") {
        Ok(home) if !home.is_empty() => PathBuf::from(home).join(".local/share/aroe"),
        _ => PathBuf::from("."),
    }
}

pub fn default_profile_path() -> PathBuf {
    data_dir().join("profile.json")
}

pub fn norm(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}
//I will js write the search part and the scoring stuff

///  i will js add a ranked title search, re-run on every keystroke: exact, then prefix, then
/// substring, then every word of the query, then a genre match. Ties lean on
/// shorter titles (the series beats its spin-offs), then the catalog's own
/// score, then the newer season.
pub fn search(catalog: &[Anime], query: &str, limit: usize) -> Vec<usize> {
    let tokens: Vec<String> = query
        .split_whitespace()
        .map(norm)
        .filter(|t| !t.is_empty())
        .collect();
    if tokens.is_empty() {
        return Vec::new();
    }
    let wanted = tokens.concat();

    let mut scored: Vec<(u8, usize, std::cmp::Reverse<u32>, std::cmp::Reverse<u32>, usize)> =
        Vec::new();
    for (i, a) in catalog.iter().enumerate() {
        let t = norm(&a.title);
        let rank = if t == wanted {
            0
        } else if t.starts_with(&wanted) {
            1
        } else if t.contains(&wanted) {
            2
        } else if tokens.iter().all(|tok| t.contains(tok)) {
            3
        } else {
            let genres: Vec<String> = a.genres.iter().map(|g| norm(g)).collect();
            if tokens
                .iter()
                .all(|tok| genres.iter().any(|g| g.contains(tok)))
            {
                4
            } else {
                continue;
            }
        };
        let popularity = (a.score.unwrap_or(0.0) * 100.0) as u32;
        scored.push((
            rank,
            t.chars().count(),
            std::cmp::Reverse(popularity),
            std::cmp::Reverse(a.year.unwrap_or(0)),
            i,
        ));
    }
    scored.sort_by(|a, b| {
        a.0.cmp(&b.0)
            .then(a.1.cmp(&b.1))
            .then(a.2.cmp(&b.2))
            .then(a.3.cmp(&b.3))
    });
    scored
        .into_iter()
        .take(limit)
        .map(|(_, _, _, _, i)| i)
        .collect()
}

pub fn find(catalog: &[Anime], title: &str) -> Option<usize> {
    let wanted = norm(title);
    catalog
        .iter()
        .position(|a| norm(&a.title) == wanted)
        .or_else(|| {
            if wanted.len() >= 5 {
                catalog
                    .iter()
                    .position(|a| norm(&a.title).contains(&wanted))
            } else {
                None
            }
        })
}

// Yh ima write code to build a users profile with the trating and stuff(it assigns scores to each)

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Rating {
    Amazing,
    Good,
    Meh,
    Bad,
}

impl Rating {
    pub const ALL: [Rating; 4] = [Rating::Amazing, Rating::Good, Rating::Meh, Rating::Bad];

    pub fn label(self) -> &'static str {
        match self {
            Rating::Amazing => "amazing",
            Rating::Good => "good",
            Rating::Meh => "meh",
            Rating::Bad => "bad",
        }
    }

    pub fn weight(self) -> f64 {
        match self {
            Rating::Amazing => 1.6,
            Rating::Good => 1.0,
            Rating::Meh => 0.45,
            Rating::Bad => -0.8,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Favorite {
    pub mal_id: u32,
    pub title: String,
    /// Older profiles stored a title without a rating; those still load.
    #[serde(default)]
    pub rating: Option<Rating>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Profile {
    pub favorites: Vec<Favorite>,
}

impl Profile {
    pub fn load(path: &Path) -> Self {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|text| serde_json::from_str(&text).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, path: &Path) {
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        if let Ok(text) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(path, text);
        }
    }

    pub fn contains(&self, mal_id: u32) -> bool {
        self.favorites.iter().any(|f| f.mal_id == mal_id)
    }

    pub fn rating_of(&self, mal_id: u32) -> Option<Rating> {
        self.favorites
            .iter()
            .find(|f| f.mal_id == mal_id)
            .and_then(|f| f.rating)
    }

    /// Save a title with its rating, or update the rating of a saved one.
    pub fn set_rating(&mut self, anime: &Anime, rating: Rating) {
        if let Some(fav) = self
            .favorites
            .iter_mut()
            .find(|f| f.mal_id == anime.mal_id)
        {
            fav.rating = Some(rating);
        } else {
            self.favorites.push(Favorite {
                mal_id: anime.mal_id,
                title: anime.title.clone(),
                rating: Some(rating),
            });
        }
    }

    /// Drop a saved title; returns true when something was removed.
    pub fn remove(&mut self, mal_id: u32) -> bool {
        let before = self.favorites.len();
        self.favorites.retain(|f| f.mal_id != mal_id);
        self.favorites.len() != before
    }
}

// Ok now the stuff for the recommender

pub struct Rec {
    pub idx: usize,
    pub score: f64,
    pub shared: Vec<String>,
}

pub fn recommend(catalog: &[Anime], profile: &Profile, limit: usize) -> Vec<Rec> {
    // Every saved title carries its rating into the taste profile: amazing
    // counts more than good, meh barely counts, and bad pulls its tags down
    // instead of up.
    let favs: Vec<(&Anime, f64)> = profile
        .favorites
        .iter()
        .filter_map(|f| {
            let rating = f.rating.unwrap_or(Rating::Good);
            catalog
                .iter()
                .find(|a| a.mal_id == f.mal_id)
                .map(|a| (a, rating.weight()))
        })
        .collect();
    if favs.is_empty() {
        return Vec::new();
    }

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
    for (f, weight) in &favs {
        for g in &f.genres {
            *taste.entry(g.as_str()).or_default() += weight * idf(g);
        }
    }

    let years: Vec<u32> = favs
        .iter()
        .filter(|(_, weight)| *weight > 0.0)
        .filter_map(|(f, _)| f.year)
        .collect();
    let avg_year = if years.is_empty() {
        None
    } else {
        Some(years.iter().sum::<u32>() as f64 / years.len() as f64)
    };
    let fav_kinds: HashSet<&str> = favs
        .iter()
        .filter(|(_, weight)| *weight > 0.0)
        .map(|(f, _)| f.kind.as_str())
        .collect();
    let fav_ids: HashSet<u32> = favs.iter().map(|(f, _)| f.mal_id).collect();
    let fav_titles: Vec<String> = favs.iter().map(|(f, _)| norm(&f.title)).collect();

    let mut recs: Vec<Rec> = catalog
        .iter()
        .enumerate()
        .filter(|(_, a)| !fav_ids.contains(&a.mal_id))
        .filter_map(|(idx, a)| {
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
            // a title built only from what you rated bad is not a pick
            if score <= 0.0 {
                return None;
            }
            if fav_kinds.contains(a.kind.as_str()) {
                score *= 1.1;
            }
            if let (Some(avg), Some(y)) = (avg_year, a.year) {
                let closeness = (1.0 - (y as f64 - avg).abs() / 20.0).max(0.0);
                score *= 1.0 + 0.15 * closeness;
            }

            shared.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap());
            Some(Rec {
                idx,
                score,
                shared: shared.into_iter().take(4).map(|(g, _)| g).collect(),
            })
        })
        .collect();

    recs.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    recs.truncate(limit);
    recs
}

// ---------- curated vibes ----------

pub const VIBES: &[(&str, &[&str])] = &[
    (
        "action",
        &[
            "Jujutsu Kaisen",
            "Frieren",
            "Attack on Titan",
            "Fullmetal Alchemist: Brotherhood",
            "One Piece",
            "Bleach",
            "Hunter x Hunter",
            "Cowboy Bebop",
            "Vinland Saga",
            "Dragon Ball",
            "Naruto",
            "Berserk",
            "Code Geass",
            "One Punch Man",
            "Gintama",
            "Kingdom",
            "Demon Slayer",
            "Neon Genesis Evangelion",
            "Mob Psycho 100",
            "Ghost in the Shell",
            "Solo Leveling",
            "Ranking of Kings",
            "Saint Seiya",
            "Inuyasha",
            "Chainsaw Man",
            "Kaiju No. 8",
            "Spy x Family",
            "Dragon Ball Super",
            "Made in Abyss",
            "Gurren Lagann",
            "Parasyte",
            "My Hero Academia",
            "86",
            "Black Clover",
            "Fate/Zero",
            "Yu-Gi-Oh!",
            "Dragon Ball Daima",
            "Dr. Stone",
            "Hell's Paradise",
            "That Time I Got Reincarnated as a Slime",
            "The Promised Neverland",
            "Dorohedoro",
            "Banana Fish",
            "Ranma 1/2",
            "Golden Kamuy",
            "Bakemonogatari",
            "Claymore",
            "Yona of the Dawn",
            "Mobile Suit Gundam Wing",
            "Shaman King",
            "Lupin the Third",
            "Fairy Tail",
            "Black Lagoon",
            "Digimon Adventure",
            "Assassination Classroom",
            "Elfen Lied",
            "Tokyo Revengers",
            "Akame ga Kill!",
            "Bungo Stray Dogs",
            "Kill la Kill",
            "Re:Zero",
        ],
    ),
    (
        "isekai",
        &[
            "Re:Zero",
            "No Game No Life",
            "Konosuba",
            "Mushoku Tensei",
            "That Time I Got Reincarnated as a Slime",
            "Overlord",
            "The Great Cleric",
            "Reincarnated as a Sword",
            "The Rising of the Shield Hero",
            "Sword Art Online",
            "The World's Finest Assassin Gets Reincarnated in Another World as an Aristocrat",
            "The Faraway Paladin",
            "Saga of Tanya the Evil",
            "World Trigger",
            "Farming Life in Another World",
            "Demon Lord, Retry!",
            "The Aristocrat's Otherworldly Adventure: Serving Gods Who Go Too Far",
            "The Irregular at Magic High School",
            "I Got a Cheat Skill in Another World and Became Unrivaled in the Real World, Too",
            "Bofuri: I Don't Want to Get Hurt, so I'll Max Out My Defense",
            "How a Realist Hero Rebuilt the Kingdom",
            "Am I Actually the Strongest?",
            "The Reincarnation of the Strongest Exorcist in Another World",
            "Black Summoner",
            "Wise Man's Grandchild",
            "The Eminence in Shadow",
            "Welcome to Demon School! Iruma-kun",
            "The 8th Son? Are You Kidding Me?",
            "In Another World with My Smartphone",
            "I'm Standing on a Million Lives",
            "My Next Life as a Villainess: All Routes Lead to Doom!",
            "Arifureta: From Commonplace to World's Strongest",
            "In the Land of Leadale",
            "Reborn as a Vending Machine, I Now Wander the Dungeon",
            "Tsukimichi: Moonlit Fantasy",
            "Skeleton Knight in Another World",
            "Trapped in a Dating Sim: The World of Otome Games Is Tough for Mobs",
            "Goblin Slayer",
            "Chivalry of a Failed Knight",
            "So I'm a Spider, So What?",
            "How NOT to Summon a Demon Lord",
            "Dead Mount Death Play",
            "The Greatest Demon Lord Is Reborn as a Typical Nobody",
            "By the Grace of the Gods",
            "Isekai Cheat Magician",
            "Kuma Kuma Kuma Bear",
            "My Unique Skill Makes Me OP Even at Level 1",
            "Sweet Reincarnation",
            "The Fruit of Evolution: Before I Knew It, My Life Had It Made!",
            "Tower of God",
            "Grimgar, Ashes and Illusions",
            "Knight's & Magic",
            "I've Been Killing Slimes for 300 Years and Maxed Out My Level",
            "Death March to the Parallel World Rhapsody",
            "The Misfit of Demon King Academy",
            "The Dungeon of Black Company",
            "High School Prodigies Have It Easy Even in Another World!",
            "Log Horizon",
            "Cautious Hero: The Hero Is Overpowered but Overly Cautious",
            "Gate",
            "I'm the Villainess, So I'm Taming the Final Boss",
            "The Strongest Sage with the Weakest Crest",
            "Uncle from Another World",
            "Plunderer",
            "Life with an Ordinary Guy Who Reincarnated into a Total Fantasy Knockout",
            "Didn't I Say to Make My Abilities Average in the Next Life?!",
            "Restaurant to Another World",
            "Shall I Survive Using Potions!",
            "Chillin' in Another World with Level 2 Super Cheat Powers",
            "Re:Monster",
            "The Weakest Tamer Began a Journey to Pick Up Trash",
            "Ascendance of a Bookworm",
        ],
    ),
    (
        "slice of life",
        &[
            "Fruits Basket",
            "Cosmic Princess Kaguya!",
            "March Comes in Like a Lion",
            "Kaguya-sama: Love Is War",
            "Clannad",
            "A Silent Voice",
            "Mob Psycho 100",
            "Sound! Euphonium",
            "Violet Evergarden",
            "Bocchi the Rock!",
            "Umamusume: Pretty Derby",
            "Umamusume: Cinderella Gray",
            "Mushi-Shi",
            "The Dangers in My Heart",
            "Look Back",
            "Nana",
            "Natsume's Book of Friends",
            "Yamada-kun and the Seven Witches",
            "Chiikawa",
            "Your Lie in April",
            "I Want to Eat Your Pancreas",
            "Grand Blue Dreaming",
            "Call of the Night",
            "The Disastrous Life of Saiki K.",
            "Wolf Children",
            "Teasing Master Takagi-san",
            "BanG Dream!",
            "Girls' Last Tour",
            "Aria the Natural",
            "The Ramparts of Ice",
            "Spice and Wolf",
        ],
    ),
    (
        "mystery",
        &[
            "Lord of Mysteries",
            "Re:Zero",
            "Death Note",
            "Attack on Titan",
            "Monogatari",
            "The Apothecary Diaries",
            "Monster",
            "JoJo's Bizarre Adventure",
            "False Memory",
            "Link Click",
            "Oshi no Ko",
            "The Disappearance of Haruhi Suzumiya",
            "Mushi-Shi: The Next Passage",
            "The Founder of Diabolism",
            "Made in Abyss",
            "Odd Taxi",
            "To Be Hero X",
            "Bungo Stray Dogs",
            "The Tatami Galaxy",
            "Madoka Magica",
            "The Garden of Sinners",
            "Pluto",
            "Demon Slayer",
            "Dorohedoro",
            "Neon Genesis Evangelion",
            "Summer Time Rendering",
            "Milgram",
            "The Promised Neverland",
            "Revolutionary Girl Utena",
            "Land of the Lustrous",
            "Made in Abyss: Wandering Twilight",
            "Scissor Seven",
            "Fate/strange Fake",
            "Mononoke the Movie",
            "Cowboy Bebop",
            "Ghost in the Shell",
            "Mushi-Shi",
            "Detective Conan",
            "Rascal Does Not Dream of Bunny Girl Senpai",
            "Honkai: Star Rail",
            "Moriarty the Patriot",
            "Mobile Police Patlabor",
            "Heavenly Delusion",
            "Baccano!",
            "Erased",
            "Princess Tutu",
            "Suzume",
            "The Tatami Time Machine Blues",
            "The Case Study of Vanitas",
            "Black Butler",
            "How to Become Ordinary",
            "Precure All Stars F",
            "The Summer Hikaru Died",
            "Toilet-Bound Hanako-kun",
            "NieR:Automata",
            "Serial Experiments Lain",
            "Little Busters! Refrain",
            "Lupin the 3rd",
            "Death Parade",
            "Haibane Renmei",
            "Higurashi: When They Cry",
            "From the New World",
            "Scott Pilgrim Takes Off",
            "Durarara!!",
            "Shadows House",
            "Kaiba",
            "Paprika",
            "Penguindrum",
            "City Hunter 2",
            "Gankutsuou: The Count of Monte Cristo",
            "Hyouka",
            "Terror in Resonance",
            "Astra Lost in Space",
            "Classroom of the Elite",
            "Sonny Boy",
            "Beastars",
            "When Marnie Was There",
            "Undead Murder Farce",
            "Dennou Coil",
            "Tsubasa",
            "Gosick",
            "Gnosia",
            "Darker than Black",
            "A Tale of Melodies",
            "Fights Break Sphere",
            "Treasure Island",
            "Magic Kaito 1412",
            "Death Billiards",
            "The Melancholy of Haruhi Suzumiya",
        ],
    ),
    (
        "romance",
        &[
            "Re:Zero",
            "Chainsaw Man",
            "Owarimonogatari",
            "Fruits Basket",
            "Kaguya-sama: Love Is War",
            "Clannad",
            "A Silent Voice",
            "The Dangers in My Heart",
            "Your Name",
            "Gurren Lagann",
            "Nana",
            "Howl's Moving Castle",
            "Rurouni Kenshin",
            "Chihayafuru",
            "The Tatami Galaxy",
            "You and I Are Polar Opposites",
            "Rascal Does Not Dream of a Dear Friend",
            "Rascal Does Not Dream of a Dreaming Girl",
            "Heaven Official's Blessing",
            "Your Lie in April",
            "Given the Movie: To the Sea",
            "The Fragrant Flower Blooms with Dignity",
            "I Want to Eat Your Pancreas",
            "Love Through a Prism",
            "Love Unseen Beneath the Clear Night Sky",
            "The Garden of Sinners",
            "Fate/stay night",
            "Call of the Night",
            "Kimi ni Todoke: From Me to You",
            "Wolf Children",
            "Bakuman",
            "Kaichou wa Maid-sama!",
            "Teasing Master Takagi-san",
            "Sounds of Life",
            "Dan Da Dan",
            "Revolutionary Girl Utena",
            "The Ramparts of Ice",
            "How to Become Ordinary",
            "Major",
            "Violet Evergarden",
            "Josee, the Tiger and the Fish",
            "Bakemonogatari",
            "Sasaki and Miyano: Graduation",
            "Steins;Gate",
            "My Teen Romantic Comedy SNAFU",
            "Is It Wrong to Try to Pick Up Girls in a Dungeon?",
            "Welcome to the N.H.K.",
            "My Dress-Up Darling",
            "Botan Kamiina Fully Blossoms When Drunk",
            "Spice and Wolf",
            "A Sign of Affection",
            "Smoking Behind the Supermarket with You",
            "Weathering with You",
            "Whisper of the Heart",
            "Into the Night",
            "Redline",
            "Blue Box",
            "Horimiya",
            "The Night Is Short, Walk On Girl",
            "Skip and Loafer",
            "ReLIFE",
            "Though I Am an Inept Villainess",
            "Gridman Universe",
            "Inuyasha",
            "The Duke of Death and His Maid",
            "Makeine: Too Many Losing Heroines!",
            "Kids on the Slope",
            "Given",
            "The Wind Rises",
            "Release That Witch",
            "Anohana: The Flower We Saw That Day",
            "The 100 Girlfriends Who Really, Really, Really, Really, Really Love You",
            "Lost in Starlight",
            "Planetes",
            "The Tunnel to Summer, the Exit of Goodbyes",
            "Insomniacs After School",
        ],
    ),
    (
        "comedy",
        &[
            "Gintama",
            "Owarimonogatari",
            "Fruits Basket",
            "Kaguya-sama: Love Is War",
            "Black Clover",
            "One Piece",
            "Mob Psycho 100",
            "Haikyu!!",
            "Hajime no Ippo",
            "Bocchi the Rock!",
            "The Dangers in My Heart",
            "Gurren Lagann",
            "Bungo Stray Dogs",
            "Dr. Stone",
            "Chiikawa",
            "Delicious in Dungeon",
            "The Tatami Galaxy",
        ],
    ),
];

/// Pick `count` curated titles for a vibe, skipping what was already shown.
pub fn vibe_picks(
    catalog: &[Anime],
    vibe: usize,
    shown: &[String],
    count: usize,
) -> Vec<usize> {
    let Some((_, titles)) = VIBES.get(vibe) else {
        return Vec::new();
    };
    let mut picked = Vec::new();
    for title in titles.iter() {
        let wanted = norm(title);
        if shown.iter().any(|s| norm(s) == wanted) {
            continue;
        }
        if let Some(idx) = catalog
            .iter()
            .position(|a| norm(&a.title) == wanted)
            .or_else(|| catalog.iter().position(|a| norm(&a.title).starts_with(&wanted)))
        {
            picked.push(idx);
            if picked.len() == count {
                break;
            }
        }
    }
    picked
}


// ---------- online details (AniList, cached) ----------

const ANILIST_URL: &str = "https://graphql.anilist.co";
const MEDIA_FIELDS: &str =
    "title { romaji english } description(asHtml: false) externalLinks { site url type }";
const DETAILS_TTL_SECS: u64 = 60 * 60 * 24 * 30;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Details {
    pub synopsis: String,
    pub streams: Vec<(String, String)>,
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

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

pub fn details_cache_path() -> PathBuf {
    data_dir().join("details.json")
}

pub fn details_key(a: &Anime) -> String {
    a.mal_id.to_string()
}

fn load_details_cache(path: &Path) -> HashMap<String, Details> {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or_default()
}


pub fn cached_details(key: &str) -> Option<Details> {
    let cache = load_details_cache(&details_cache_path());
    cache
        .get(key)
        .filter(|d| now_secs().saturating_sub(d.fetched_at) < DETAILS_TTL_SECS)
        .cloned()
}

pub fn store_details(key: &str, details: &Details) {
    let path = details_cache_path();
    let mut cache = load_details_cache(&path);
    cache.insert(key.to_string(), details.clone());
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    if let Ok(text) = serde_json::to_string(&cache) {
        let _ = std::fs::write(&path, text);
    }
}

fn post_graphql(body: serde_json::Value) -> Option<AniListMedia> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("aroe/0.4")
        .timeout(std::time::Duration::from_secs(12))
        .build()
        .ok()?;
    let response: AniListResponse = client
        .post(ANILIST_URL)
        .json(&body)
        .send()
        .ok()?
        .json()
        .ok()?;
    response.data?.media
}

fn titles_match(expected: &str, media: &AniListMedia) -> bool {
    let want = norm(expected);
    [media.title.romaji.as_deref(), media.title.english.as_deref()]
        .into_iter()
        .flatten()
        .any(|t| norm(t) == want)
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
        .filter(|l| !l.is_empty())
        .filter(|l| {
            let lower = l.to_lowercase();
            !lower.starts_with("note:") && !lower.starts_with("(source") && !lower.starts_with("source:")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn details_from_media(media: AniListMedia) -> Details {
    Details {
        synopsis: clean_description(media.description.as_deref().unwrap_or("")),
        streams: media
            .external_links
            .unwrap_or_default()
            .into_iter()
            .filter(|l| l.kind.as_deref() == Some("STREAMING"))
            .map(|l| (l.site, l.url))
            .collect(),
        fetched_at: now_secs(),
    }
}


pub fn fetch_details(anime: &Anime) -> Option<Details> {
    let mut media = None;

    if let Some(id) = anime.anilist_id {
        media = post_graphql(serde_json::json!({
            "query": format!("query($id:Int){{ Media(id:$id) {{ {} }} }}", MEDIA_FIELDS),
            "variables": { "id": id }
        }));
    }
    if media.is_none() {
        media = post_graphql(serde_json::json!({
            "query": format!("query($idMal:Int){{ Media(idMal:$idMal) {{ {} }} }}", MEDIA_FIELDS),
            "variables": { "idMal": anime.mal_id }
        }));
    }
    if media
        .as_ref()
        .map_or(true, |m| !titles_match(&anime.title, m))
    {
        if let Some(found) = post_graphql(serde_json::json!({
            "query": format!("query($search:String){{ Media(search:$search, type:ANIME) {{ {} }} }}", MEDIA_FIELDS),
            "variables": { "search": anime.title }
        })) {
            media = Some(found);
        }
    }

    media.map(details_from_media)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bundled() -> Option<Vec<Anime>> {
        let path = Path::new("data/data.json");
        if !path.exists() {
            return None;
        }
        Some(load(path).expect("bundled catalog parses"))
    }

    #[test]
    fn catalog_loads_and_is_slimmed() {
        let Some(catalog) = bundled() else { return };
        assert!(catalog.len() > 5_000, "catalog looks too small");
        assert!(catalog.iter().all(|a| !a.title.is_empty()));
        assert!(catalog.iter().any(|a| !a.genres.is_empty()));
    }

    #[test]
    fn search_finds_known_titles_first() {
        let Some(catalog) = bundled() else { return };
        let hits = search(&catalog, "frieren", 20);
        assert!(!hits.is_empty());
        let best = &catalog[hits[0]].title;
        assert!(
            best.to_lowercase().contains("frieren"),
            "best hit was {best}"
        );
        // the main series should outrank its longer spin-off titles
        assert!(
            !best.contains(':'),
            "expected the main series first, got {best}"
        );

        let genre_hits = search(&catalog, "isekai", 20);
        assert!(!genre_hits.is_empty());
    }

    #[test]
    fn recommendations_explain_themselves() {
        let Some(catalog) = bundled() else { return };
        let Some(idx) = search(&catalog, "frieren", 1).first().copied() else {
            return;
        };
        let mut profile = Profile::default();
        profile.set_rating(&catalog[idx], Rating::Good);

        let recs = recommend(&catalog, &profile, 10);
        assert!(!recs.is_empty(), "expected picks for a popular title");
        assert!(recs.iter().all(|r| r.shared.len() >= 2));
        assert!(recs.iter().all(|r| r.idx != idx));
    }

    #[test]
    fn ratings_steer_the_profile() {
        let Some(catalog) = bundled() else { return };
        let Some(idx) = search(&catalog, "frieren", 1).first().copied() else {
            return;
        };
        let mut profile = Profile::default();
        profile.set_rating(&catalog[idx], Rating::Amazing);
        assert_eq!(profile.rating_of(catalog[idx].mal_id), Some(Rating::Amazing));
        let loved = recommend(&catalog, &profile, 10);
        assert!(!loved.is_empty(), "an amazing rating should produce picks");

        profile.set_rating(&catalog[idx], Rating::Bad);
        assert_eq!(profile.rating_of(catalog[idx].mal_id), Some(Rating::Bad));
        let disliked = recommend(&catalog, &profile, 10);
        assert!(
            disliked.len() < loved.len(),
            "a bad rating should pull its tags out of the picks"
        );

        assert!(profile.remove(catalog[idx].mal_id));
        assert!(profile.rating_of(catalog[idx].mal_id).is_none());
    }

    #[test]
    fn profiles_written_before_ratings_still_load() {
        let text = r#"{"favorites":[{"mal_id":1,"title":"x"}]}"#;
        let profile: Profile = serde_json::from_str(text).expect("old profile parses");
        assert_eq!(profile.favorites.len(), 1);
        assert_eq!(profile.favorites[0].rating, None);
    }

    #[test]
    fn vibes_resolve_to_catalog_titles() {
        let Some(catalog) = bundled() else { return };
        let picks = vibe_picks(&catalog, 0, &[], 3);
        assert_eq!(picks.len(), 3);

        let shown: Vec<String> = picks.iter().map(|i| catalog[*i].title.clone()).collect();
        let next = vibe_picks(&catalog, 0, &shown, 3);
        assert!(next.iter().all(|i| !picks.contains(i)));
    }
}
