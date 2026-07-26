fn main() {
    struct Excerpt<'a> { part: &'a str }
    impl<'a> Excerpt<'a> {
        fn new(part: &'a str) -> Self { Excerpt { part } }
    }
    let novel = String::from("Call me Ishmael. Call me Ishmael again.");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = Excerpt::new(first_sentence);
    println!("Excerpt: {}", excerpt.part);
}
