fn main() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }
    let s1 = String::from("short");
    let s2 = String::from("loooooong");
    let result = longest(&s1, &s2);
    println!("Longest: {result}");
}
