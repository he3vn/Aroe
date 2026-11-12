mod data;

use inquire::Select;
use std::io::{self, Write};

// This is basically my new input= for rust since they dont have it
fn ask(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn main() {
    let line = "=".repeat(30);
    println!("{}", line);
    println!("   Welcome to AnimeRo");
    println!("{}", line);
    println!("  Your anime recommender");
    println!("{}", "-".repeat(30));

    let action=vec![
        "Jujutsu Kaisen",
        "Frieren",
        "Attack on Titan",
        "Fullmetal Alchemist: Brotherhood",
        "One piece",
        "Bleach",
        "Hunter X Hunter",
        "Cowboy Bebop",
        "Vinland Saga",
        "Dragon ball",
        "Naruto",
        "Berserk",
        "Code geass",
        "One punch man",
        "Gintama",
        "Kingdom",
        "Demon Slayer",
        "Neone Genesis Evangelion",
        "Mob psycho 100",
        "Ghost in the shell",
        "Solo leveling",
        "Ranking of kings",
        "Saint Seya",
        "InuYusha",
        "Chainsaw Man",
        "Kaiju n.8",
        "Spy x family",
        "Dragon ball Super",
        "Made in Abyss",
        "Gurren laggan",
        "Parasyte",
        "My hero academia",
        "86",
        "Black clover",
        "Fate/Zero",
        "Yu-Gi-Oh",
        "Dragon ball Daima",
        "Dr.Stone",
        "Hell's Paradise",
        "Tensura(That Time I got reincarnated as a slime)",
        "The promised Neverland",
        "Doredoro",
        "Banana Fish",
        "Ranma 1/2",
        "Golden Kamuy",
        "Bakemonogatari",
        "Claymore",
        "Yona of the Dawn",
        "Mobile Suit Gundan Wing",
        "Shaman King",
        "Lupin the Third",
        "Fairy Tail",
        "Black Lagoon",
        "Digimon Adventure",
        "Assasination Classroom",
        "Elfen lied",
        "Tokyo Revengers",
        "Akame ga Kill",
        "Bungo Stray Dogs",
        "Kill la kill",
        "Re:zero",
    ];
    let isekai=vec![
        "Re:Zero",
        "No game no life",
        "Konosuba",
        "Mushoku Tensei",
        "Tensura(That Time I got reincarnated as a slime)",
        "Overlord",
        "The Great Cleric",
        "Reincarnated as a Sword",
        "The Rising of the Shield Hero",
        "Sword Art Online",
        "The World's Finest Assasin Gets Reincarnated in Another World as an Aristocrat",
        "The Faraway Paladin",
        "Saga of Tanya the Evil",
        "World Trigger",
        "Farming life in Another World",
        "Demon lord,Retry!",
        "The Aristocrat's Otherwordly Adventure: Serving Gods Who Go Too Far",
        "The Irregular at Magic High school",
        "I got a Cheat skill in Another World and Became unrivaled in the Real world,Too",
        "Bofuri:I Don't want to Get Hurt,So I'll Max Out My Defense",
        "How a Realist Hero Rebuilt the Kingdom",
        "Am I Actually the Strongest?",
        "The Reincarnationf of the Strongest Exorcist in Another World",
        "Black Summoner",
        "Wise Man's Grandchild",
        "The Eminence in Shadow",
        "Welcome to Demon-school,Iruma-kun",
        "The 8th Son?Are you Kidding Me?",
        "In Another World with my Smartphone",
        "Im Standing on a Million Lives",
        "My Next Life as a Vilainess:All Routes Lead to Doom",
        "Arifureta:From Commonplace to World's Strongest",
        "In the Land of Leadale",
        "Reborn as a Vednding Machine,Now I Wander the Dungeon",
        "Tsukimichi:Moonlit Fantasy",
        "Skeleton Knight in Another World",
        "Trapped in a Dating Sim:The World of Otome Games is Tough for Mobs",
        "Goblin Slayer",
        "Chivalry of a Failed knight",
        "So I'm a spider,So What?",
        "How NOT to Summon a demon lord",
        "Dead Mount Death Play",
        "The Greatest Demon Lord is Reborn as a Typical Nobody",
        "By the grace of the Gods",
        "Isekai Cheat Magician",
        "Kuma Kuma Kuma Bear",
        "My Unique Skill Makes Me OP Even at Level 1",
        "Sweet Reincarnation",
        "The Fruit of Evolution:Before I Knew It,My Life Had It Made",
        "Tower of God",
        "Grimgar,Ashes and Illusions",
        "Knight's & Magic",
        "I've Been Killing Slimes for 300 Years and Maexed out My Level",
        "Death March to the Parallel World Rhaspody",
        "The Misfit of Demon King Academy",
        "The Dungeon of Black Company",
        "High School Prodigies Have It Easy even in Another World!",
        "Log Horizon",
        "Cautious Hero:The Hero is Overpowered but Overly Cautious",
        "Gate",
        "I'm the Vilainess,So I'm Taming the Final Boss",
        "The Strongest Sage with the Weakest Crest",
        "Uncle from Anotehr World",
        "Plunderer",
        "Life with an Ordinary Guy Who Reincarnated into a Total Fantasy Knockout!",
        "Didnt I say to Make my Abilities Average in the Next Life?!",
        "Restaurant to Another World",
        "Shall I Survive Using Potions!",
        "Chilling in Another World with Level 2 Super Cheat Powers",
        "Re:Monster",
        "The Weakest Tamer Began a Journey to Pick Up Trash",
        "Ascendance of a Bookworm",
    ];





    

    // Load catalog from database
    let anime_catalog = data::catalog("data.json");

    let name = ask("Are you new to anime?(Yes/No): ");

    // Lets start the logic this is going to be long I js realised i need to scrape a crazy amount of stuff to make this
    if name.eq_ignore_ascii_case("Yes") {
        let genre = ask(
            "What genre do you like?:\n\
             1. Action(Shonen)\n\
             2. Isekai/Fantasy\n\
             3. Slice of Life(Vibe/Relax)\n\
             4. Fantasy\n\
             5. Romance\n\
             Enter choice: ",
        );

        // lowercase + trim once, so "2", "Isekai", "isekai fantasy" all work
        let genre = genre.trim().to_lowercase();

        if genre == "1" || genre.contains("action") || genre.contains("shonen") {
            let nanos= std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .subsec_nanos() as usize;
            
            let pick=action[nanos % action.len()];
            println!("Recommended: {}",pick);
        } else if genre == "2" || genre.contains("isekai") {
            let nanos= std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .subsec_nanos() as usize;
            
            let pick=isekai[nanos % isekai.len()];
            println!("Recommended: {}",pick);
        } else if genre == "3" || genre.contains("slice of life") || genre.contains("vibe") || genre.contains("relax") {
            println!("Recommended: Spy x Family!");
        } else if genre == "4" || genre.contains("fantasy") {
            println!("Recommended: Frieren: Beyond Journey's End!");
        } else if genre == "5" || genre.contains("romance") {
            println!("Recommended: Kaguya-sama: Love Is War!");
        } else {
            println!("Recommended: Fullmetal Alchemist: Brotherhood!");
        }
    } else {
        println!("\nSearch through the catalog:");

        let titles: Vec<String> = anime_catalog
            .iter()
            .map(|a| a.title.clone())
            .collect();

        if titles.is_empty() {
            println!("No anime found in database!");
            return;
        }

        let selection = Select::new("Type anime title:", titles)
            .with_page_size(10)
            .prompt();

        match selection {
            Ok(chosen_title) => {
                if let Some(anime) = anime_catalog.iter().find(|a| a.title == chosen_title) {
                    println!("\n{}", "=".repeat(30));
                    println!("Title: {}", anime.title);
                    println!("Type: {}", anime.kind);
                    if let Some(eps) = anime.episodes {
                        println!("Episodes: {}", eps);
                    }
                    if let Some(year) = anime.year {
                        println!("Year: {}", year);
                    }
                    if !anime.genres.is_empty() {
                        println!("Tags: {}", anime.genres.join(", "));
                    }
                    println!("{}", "=".repeat(30));

                    match data::details(anime) {
                        Some(details) => {
                            if details.synopsis.is_empty() {
                                println!("Synopsis: Not available");
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
            }
            Err(_) => {
                println!("Search cancelled.");
            }
        }
    }
}