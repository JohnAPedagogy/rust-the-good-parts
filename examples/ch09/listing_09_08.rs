fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let evens: Vec<i32> = v.iter().filter(|&&x| x % 2 == 0).copied().collect();
    println!("Evens: {evens:?}");
    let squares: Vec<i32> = v.iter().map(|&x| x * x).collect();
    println!("Squares: {squares:?}");
    let sum: i32 = v.iter().fold(0, |acc, &x| acc + x);
    println!("Sum: {sum}");
}
