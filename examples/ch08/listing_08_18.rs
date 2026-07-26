fn main() {
    trait Summary { fn summarise(&self) -> String; }
    struct Tweet { username: String, content: String }
    impl Summary for Tweet {
        fn summarise(&self) -> String { format!("@{}: {}", self.username, self.content) }
    }
    fn make_tweet() -> impl Summary {
        Tweet { username: String::from("rustlang"), content: String::from("Coming soon!") }
    }
    let t = make_tweet();
    println!("{}", t.summarise());
}
