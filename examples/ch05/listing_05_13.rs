fn main() {
    #[derive(Debug, Clone, Copy)]
    struct Point { x: f64, y: f64 }
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1;
    println!("{:?} {:?}", p1, p2);
}
