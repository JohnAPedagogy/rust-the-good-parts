use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    *map.entry("rust").or_insert(0) += 1;
    *map.entry("rust").or_insert(0) += 1;
    println!("rust count: {}", map["rust"]);
}
