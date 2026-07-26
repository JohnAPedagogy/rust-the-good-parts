fn main() {
    let scores = [10, 20, 30, 40];
    for score in &scores {
        println!("{score}");
    }
    println!("Array still available: {:?}", scores);
}
