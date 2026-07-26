fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = nums.iter().map(|&x| x * 2).collect();
    println!("Vec: {doubled:?}");
    let as_strings: String = nums.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
    println!("Joined: {as_strings}");
}
