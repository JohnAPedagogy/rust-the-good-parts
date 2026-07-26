fn main() {
    let fixed: [i32; 3] = [10, 20, 30];
    let mut growable: Vec<i32> = vec![10, 20, 30];
    growable.push(40);
    println!("array len: {}", fixed.len());
    println!("vec len:   {}", growable.len());
}
