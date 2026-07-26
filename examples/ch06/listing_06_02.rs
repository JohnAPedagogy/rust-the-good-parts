fn main() {
    let v = vec![10, 20, 30, 40, 50];
    println!("Third: {}", v[2]);
    match v.get(10) {
        Some(val) => println!("Found: {val}"),
        None => println!("Index out of range"),
    }
    let middle: &[i32] = &v[1..4];
    println!("Middle: {middle:?}");
}
