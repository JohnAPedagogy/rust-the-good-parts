fn main() {
    let scores = [10, 20, 30, 40, 50];
    println!("First score: {}", scores[0]);
    println!("Number of scores: {}", scores.len());
    for score in scores {
        println!("{score}");
    }
}
