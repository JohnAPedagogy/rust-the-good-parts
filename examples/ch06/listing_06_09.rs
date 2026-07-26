use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, u32> = HashMap::new();
    let name = String::from("Alice");
    scores.insert(name, 95);
    println!("{:?}", scores.get("Alice"));
}
