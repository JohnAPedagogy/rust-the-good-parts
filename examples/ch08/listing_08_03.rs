fn main() {
    trait Summary {
        fn summarise_author(&self) -> String;
        fn summarise(&self) -> String {
            format!("(Read more from {}...)", self.summarise_author())
        }
    }
    struct Tweet { username: String, content: String }
    impl Summary for Tweet {
        fn summarise_author(&self) -> String { format!("@{}", self.username) }
    }
    let tweet = Tweet { username: String::from("rustlang"), content: String::from("Exciting!") };
    println!("{}", tweet.summarise());
}
