fn main() {
    let mut scores: Vec<i32> = Vec::new();
    scores.push(10);
    scores.push(20);
    scores.push(30);
    println!("Length: {}", scores.len());
    if let Some(last) = scores.pop() {
        println!("Popped: {last}");
    }
    let primes = vec![2, 3, 5, 7, 11];
    println!("Primes: {primes:?}");
}
