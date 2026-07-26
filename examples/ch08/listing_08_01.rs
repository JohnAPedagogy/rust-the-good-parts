fn main() {
    trait Summary {
        fn summarise(&self) -> String;
    }
    struct NewsArticle { headline: String }
    struct Tweet { username: String, content: String }
    impl Summary for NewsArticle {
        fn summarise(&self) -> String { self.headline.clone() }
    }
    impl Summary for Tweet {
        fn summarise(&self) -> String {
            format!("@{}: {}", self.username, self.content)
        }
    }
    let article = NewsArticle { headline: String::from("Rust tops survey") };
    let tweet = Tweet { username: String::from("rustlang"), content: String::from("Exciting!") };
    println!("{}", article.summarise());
    println!("{}", tweet.summarise());
}
