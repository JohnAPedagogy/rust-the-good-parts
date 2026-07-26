fn main() {
    let raw = vec!["3", "", "7", "bad", "2", "9", "", "4"];
    let result: Vec<u32> = raw.iter()
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<u32>().ok())
        .map(|n| n * n)
        .collect();
    println!("{result:?}");
}
