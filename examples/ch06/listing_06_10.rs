fn main() {
    let mut s = String::new();
    s.push_str("Hello");
    s.push_str(", world");
    s.push('!');
    println!("{s}");
    let s1 = String::from("Hello");
    let s2 = String::from(", world!");
    let s3 = s1 + &s2;
    println!("{s3}");
    let a = String::from("tic");
    let b = String::from("tac");
    let c = String::from("toe");
    println!("{}", format!("{a}-{b}-{c}"));
}
