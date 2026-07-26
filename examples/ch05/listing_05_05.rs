fn main() {
    struct Player { name: String, score: u32 }
    let p = Player {
        name: String::from("Alice"),
        score: 42,
    };
    let Player { name, score } = &p;
    println!("{name} has {score} points");
    let Player { ref name, .. } = p;
    println!("Name: {name}");
}
