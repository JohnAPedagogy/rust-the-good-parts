fn main() {
    trait Summary { fn summarise(&self) -> String; }
    struct NewsArticle { headline: String }
    struct Tweet { username: String, content: String }
    impl Summary for NewsArticle {
        fn summarise(&self) -> String { self.headline.clone() }
    }
    impl Summary for Tweet {
        fn summarise(&self) -> String { format!("@{}: {}", self.username, self.content) }
    }
    let items: Vec<Box<dyn Summary>> = vec![
        Box::new(NewsArticle { headline: String::from("Rust tops survey") }),
        Box::new(Tweet { username: String::from("rustlang"), content: String::from("Exciting!") }),
    ];
    for item in &items { println!("{}", item.summarise()); }
}
