fn main() {
    let csv = "Alice,92,88,75";
    let parts: Vec<&str> = csv.split(',').collect();
    println!("{parts:?}");
    let num: i32 = "42".parse().expect("not a number");
    println!("Parsed: {num}");
}
