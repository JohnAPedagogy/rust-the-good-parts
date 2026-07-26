fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let total = v.iter().fold(0, |acc, &x| acc + x);
    println!("Total: {total}");
    let sum: i32 = v.iter().sum();
    println!("Sum: {sum}");
}
