mod data;

use inquire::Select;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

fn ask(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

struct Rng(u64);

impl Rng {
    fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);
        // xorshift must never start at 0
        Rng((seed ^ 0x9E37_79B9_7F4A_7C15) | 1)
    }

    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    fn below(&mut self, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            (self.next() % n as u64) as usize
        }
    }
}

fn pick_unique(rng: &mut Rng, list: &[&str], count: usize, shown: &[String]) -> Vec<String> {
    let mut picks: Vec<String> = Vec::new();

    if list.is_empty() {
        return picks;
    }

    let mut tries = 0;
    while picks.len() < count && tries < count * 50 {
        tries += 1;
        let candidate = list[rng.below(list.len())];

        let already_shown = shown.iter().any(|s| s.eq_ignore_ascii_case(candidate));
        let already_picked = picks.iter().any(|s| s.eq_ignore_ascii_case(candidate));

        if !already_shown && !already_picked {
            picks.push(candidate.to_string());
        }
    }

    picks
}

fn norm(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

fn find_anime<'a>(catalog: &'a [data::Anime], wanted: &str) -> Option<&'a data::Anime> {
    // 1. exact, ignoring case
    if let Some(a) = catalog.iter().find(|a| a.title.eq_ignore_ascii_case(wanted)) {
        return Some(a);
    }

    let want = norm(wanted);
    if want.is_empty() {
        return None;
    }

    // 2. exact after normalizing punctuation/spacing
    if let Some(a) = catalog.iter().find(|a| norm(&a.title) == want) {
        return Some(a);
    }

    // 3. loose "contains" match (only for reasonably long queries)
    if want.len() >= 5 {
        return catalog.iter().find(|a| norm(&a.title).contains(&want));
    }

    None
}

fn print_details(details: Option<data::Details>) {
    match details {
        Some(details) => {
            if details.synopsis.is_empty() {
                println!("\nSynopsis: Not available");
            } else {
                println!("\nSynopsis:\n{}", details.synopsis);
            }

            if details.streams.is_empty() {
                println!("\nWhere to watch: Not available");
            } else {
                println!("\nWhere to watch:");
                for (site, url) in &details.streams {
                    println!("  {} — {}", site, url);
                }
            }
        }
        None => {
            println!("\nSynopsis: Not available");
            println!("Where to watch: Not available");
        }
    }
}

fn show(title: &str, catalog: &[data::Anime]) {
    match find_anime(catalog, title) {
        Some(anime) => {
            println!("\n{}", "=".repeat(30));
            println!("Title: {}", anime.title);
            println!("Type: {}", anime.kind);
            if let Some(eps) = anime.episodes {
                println!("Episodes: {}", eps);
            }
            if let Some(year) = anime.year {
                println!("Year: {}", year);
            }
            println!("{}", "=".repeat(30));

            print_details(data::details(anime));
        }
        None => {
            println!("\n{}", "=".repeat(30));
            println!("Title: {}", title);
            println!("{}", "=".repeat(30));

            print_details(data::details_by_title(title));
        }
    }
}

fn main() {
    let line = "=".repeat(30);
    println!("{}", line);
    println!("   Welcome to AnimeRo");
    println!("{}", line);
    println!("  Your anime recommender");
    println!("{}", "-".repeat(30));

    let action = vec![
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
    ];

    let isekai = vec![
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
    ];

    let mystery = vec![
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
    ];

    let romance = vec![
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
    ];

    let comedy = vec![
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
    ];

    let sliceoflife = vec![
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
    ];

    // Load catalog from database
    let anime_catalog = data::catalog("data.json");

    let name = ask("Are you new to anime? (Yes/No): ");

    if name.eq_ignore_ascii_case("yes") || name.eq_ignore_ascii_case("y") {
        let genre = ask(
            "What genre do you like?:\n\
             1. Action (Shonen)\n\
             2. Isekai/Fantasy\n\
             3. Slice of Life (Vibe/Relax)\n\
             4. Mystery\n\
             5. Romance\n\
             6. Comedy\n\
             Enter choice: ",
        );

        let genre = genre.trim().to_lowercase();

        let list: &[&str] = if genre == "1" || genre.contains("action") || genre.contains("shonen")
        {
            &action
        } else if genre == "2" || genre.contains("isekai") || genre.contains("fantasy") {
            &isekai
        } else if genre == "3"
            || genre.contains("slice of life")
            || genre.contains("vibe")
            || genre.contains("relax")
        {
            &sliceoflife
        } else if genre == "4" || genre.contains("mystery") {
            &mystery
        } else if genre == "5" || genre.contains("romance") {
            &romance
        } else if genre == "6" || genre.contains("comedy") {
            &comedy
        } else {
            println!("Recommended: Re:Zero");
            return;
        };

        if list.is_empty() {
            println!("That list is empty, add some titles first.");
            return;
        }

        let mut rng = Rng::new();
        let mut shown: Vec<String> = Vec::new();

        loop {
            let picks = pick_unique(&mut rng, list, 3, &shown);

            if picks.is_empty() {
                println!("\nNo more anime to show for this genre.");
                break;
            }

            for title in &picks {
                show(title, &anime_catalog);
                shown.push(title.clone());
            }

            if picks.len() < 3 {
                println!("\nThat's all I have for this genre.");
                break;
            }

            let more = ask("\nWant more? (Yes/No): ");
            if !more.eq_ignore_ascii_case("yes") && !more.eq_ignore_ascii_case("y") {
                break;
            }
        }
    } else {
        println!("\nSearch through the catalog:");

        let titles: Vec<String> = anime_catalog.iter().map(|a| a.title.clone()).collect();

        if titles.is_empty() {
            println!("No anime found in database!");
            return;
        }

        let selection = Select::new("Type anime title:", titles)
            .with_page_size(10)
            .prompt();

        match selection {
            Ok(chosen_title) => show(&chosen_title, &anime_catalog),
            Err(_) => {
                println!("Search cancelled.");
            }
        }
    }
}