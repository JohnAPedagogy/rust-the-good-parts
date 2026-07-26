use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("Alice", 95);
    map.insert("Bob", 82);
    println!("Alice: {:?}", map.get("Alice"));
    println!("Contains Charlie: {}", map.contains_key("Charlie"));
    for (name, score) in &map { println!("{name}: {score}"); }
}
