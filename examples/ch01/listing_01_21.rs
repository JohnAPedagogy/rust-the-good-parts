fn main() {
    struct Player { name: String, score: u32, active: bool }
    let p = Player {
        name: String::from("Alice"),
        score: 0,
        active: true,
    };
    if p.active {
        println!("{} has {} points", p.name, p.score);
    }
}
