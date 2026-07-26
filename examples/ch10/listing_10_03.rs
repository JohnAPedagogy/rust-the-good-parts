use std::fmt;

fn main() {
    #[derive(Debug)]
    struct Point<T> { x: T, y: T }
    impl<T: fmt::Display> Point<T> {
        fn display(&self) { println!("({}, {})", self.x, self.y); }
    }
    impl Point<f64> {
        fn distance_from_origin(&self) -> f64 { (self.x * self.x + self.y * self.y).sqrt() }
    }
    let ip = Point { x: 5, y: 10 };
    let fp = Point { x: 1.0, y: 4.0 };
    ip.display();
    fp.display();
    println!("Distance: {:.4}", fp.distance_from_origin());
}
