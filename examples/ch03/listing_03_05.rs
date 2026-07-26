fn main() {
    fn summarise(a: f64, b: f64, c: f64, d: f64) -> f64 {
        fn total(a: f64, b: f64, c: f64, d: f64) -> f64 {
            a + b + c + d
        }
        total(a, b, c, d) / 4.0
    }
    println!("Average: {}", summarise(10.0, 20.0, 30.0, 40.0));
}
