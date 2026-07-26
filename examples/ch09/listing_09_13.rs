fn main() {
    #[derive(Debug)]
    struct Point { x: f64, y: f64 }
    impl Point {
        fn dist(&self) -> f64 { (self.x * self.x + self.y * self.y).sqrt() }
    }
    let mut points = vec![
        Point { x: 3.0, y: 4.0 },
        Point { x: 1.0, y: 1.0 },
        Point { x: 0.0, y: 2.0 },
    ];
    points.sort_by(|a, b| a.dist().partial_cmp(&b.dist()).unwrap());
    for p in &points { println!("{:.2}", p.dist()); }
}
