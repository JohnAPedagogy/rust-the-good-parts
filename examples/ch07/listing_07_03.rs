fn main() {
    let n = "42".parse::<i32>().unwrap();
    println!("unwrap: {n}");
    let m = "42".parse::<i32>().expect("Not a valid number");
    println!("expect: {m}");
}
