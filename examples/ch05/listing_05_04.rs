fn main() {
    struct Player { name: String, score: u32, level: u32 }
    let alice = Player {
        name: String::from("Alice"),
        score: 0,
        level: 1,
    };
    let alice_promoted = Player { level: 2, ..alice };
    println!("Level: {}", alice_promoted.level);
    println!("Score: {}", alice_promoted.score);
}
