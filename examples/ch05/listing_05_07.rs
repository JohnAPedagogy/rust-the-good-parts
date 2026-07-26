fn main() {
    struct Counter { count: u32, max: u32 }
    impl Counter {
        fn new(max: u32) -> Self { Counter { count: 0, max } }
        fn increment(&mut self) -> bool {
            if self.count < self.max { self.count += 1; true } else { false }
        }
        fn value(&self) -> u32 { self.count }
    }
    let mut c = Counter::new(3);
    c.increment(); c.increment(); c.increment();
    println!("Count: {}, overflowed: {}", c.value(), c.increment());
}
