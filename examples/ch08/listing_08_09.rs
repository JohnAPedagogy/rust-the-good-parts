use std::fmt;

fn main() {
    #[derive(Debug)]
    struct Point { x: f64, y: f64 }
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    let p = Point { x: 3.0, y: -1.5 };
    println!("Display: {}", p);
    println!("Debug: {:?}", p);
}
