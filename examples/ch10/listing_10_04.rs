use std::fmt;

fn main() {
    #[derive(Debug)]
    struct Pair<T, U> { first: T, second: U }
    impl<T: fmt::Display, U: fmt::Display> Pair<T, U> {
        fn show(&self) { println!("first: {}, second: {}", self.first, self.second); }
    }
    let p = Pair { first: 42, second: "hello" };
    p.show();
}
