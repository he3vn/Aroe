// This is going to be my new input might add some others to make it feel like python
use std::io::{self, Write};

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

    let name = ask("Are you new to anime?(Yes/No): ");

    // Lets start the logic this is going to be longgg
    if name == "Yes" {
        let genre = ask(
            "What genre do you like?:\n\
             1. Action(Shonen)\n\
             2. Mystery & Thriller\n\
             3. Slice of Life(Vibe/Relax)\n\
             4. Fantasy\n\
             5. Romance\n\
             Enter choice: "
        );

        if genre == "1" || genre == "Action" {
            println!("Recommended: Attack on Titan!");
        }
    }
}