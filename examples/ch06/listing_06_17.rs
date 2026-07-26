fn main() {
    let v = vec![2, 4, 6, 8, 10];
    println!("All even: {}", v.iter().all(|&x| x % 2 == 0));
    println!("Any > 9: {}", v.iter().any(|&x| x > 9));
    println!("First > 5: {:?}", v.iter().find(|&&x| x > 5));
}
