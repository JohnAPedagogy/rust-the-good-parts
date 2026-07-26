fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = v.iter().map(|&x| x * 2).collect();
    println!("{doubled:?}");
}
