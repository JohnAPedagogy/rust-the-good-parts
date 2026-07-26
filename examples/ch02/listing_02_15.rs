fn main() {
    let scores = [8, 9, 7, 10, 9];
    let all_passing = scores.iter().all(|&s| s > 5);
    println!("{all_passing}");
    let all_perfect = scores.iter().all(|&s| s == 10);
    println!("{all_perfect}");
}
