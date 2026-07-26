fn main() {
    struct Rectangle { width: f64, height: f64 }
    impl Rectangle {
        fn new(width: f64, height: f64) -> Self { Rectangle { width, height } }
        fn square(size: f64) -> Self { Rectangle { width: size, height: size } }
        fn area(&self) -> f64 { self.width * self.height }
    }
    let r = Rectangle::new(10.0, 5.0);
    let s = Rectangle::square(4.0);
    println!("Rect area: {}, Square area: {}", r.area(), s.area());
}
