fn main() {
    trait Summary { fn summarise(&self) -> String; }
    struct Article { headline: String }
    impl Summary for Article {
        fn summarise(&self) -> String { self.headline.clone() }
    }
    fn notify_impl(item: &impl Summary) {
        println!("Breaking: {}", item.summarise());
    }
    fn notify_generic<T: Summary>(item: &T) {
        println!("Breaking: {}", item.summarise());
    }
    let a = Article { headline: String::from("News!") };
    notify_impl(&a);
    notify_generic(&a);
}
