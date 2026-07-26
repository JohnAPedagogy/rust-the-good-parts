fn main() {
    struct Player { name: String, score: u32 }
    let name = String::from("Alice");
    let score = 0;
    let p1 = Player { name: name.clone(), score };
    let name = String::from("Bob");
    let p2 = Player { name, score };
    println!("{} vs {}", p1.name, p2.name);
}
