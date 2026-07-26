fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    v.retain(|&x| x % 2 == 0);
    println!("Evens: {v:?}");
    let data = vec![10, 20, 30, 40, 50];
    for window in data.windows(3) {
        println!("Window: {window:?}");
    }
}
