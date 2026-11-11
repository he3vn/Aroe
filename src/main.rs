mod data;

use inquire::Select;
use std::{io::{self, Write}, time::Duration};

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



    // Load catalog from database
    let anime_catalog = data::catalog("data.json");

    let name = ask("Are you new to anime?(Yes/No): ");

    // Lets start the logic this is going to be long I js realised i need to scrape a crazy amount of stuff to make this
    if name.eq_ignore_ascii_case("Yes") {
        let genre = ask(
            "What genre do you like?:\n\
             1. Action(Shonen)\n\
             2. Mystery & Thriller\n\
             3. Slice of Life(Vibe/Relax)\n\
             4. Fantasy\n\
             5. Romance\n\
        so     Enter choice: ",
        );

        if genre == "1" || genre.eq_ignore_ascii_case("Action") {
            let nanos= std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .subsec_nanos() as usize;
            
            let pick=action[nanos % action.len()];
            println!("Recommended: {}",pick);
        } else if genre == "2" || genre.eq_ignore_ascii_case("Mystery") {
            println!("Recommended: Death Note!");
        } else if genre == "3" || genre.eq_ignore_ascii_case("Slice of Life") {
            println!("Recommended: Spy x Family!");
        } else if genre == "4" || genre.eq_ignore_ascii_case("Fantasy") {
            println!("Recommended: Frieren: Beyond Journey's End!");
        } else if genre == "5" || genre.eq_ignore_ascii_case("Romance") {
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