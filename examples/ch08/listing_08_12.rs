use std::ops::Add;

fn main() {
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Point { x: f64, y: f64 }
    impl Add for Point {
        type Output = Point;
        fn add(self, other: Point) -> Point {
            Point { x: self.x + other.x, y: self.y + other.y }
        }
    }
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: 3.0, y: 4.0 };
    println!("{:?}", p1 + p2);
}
