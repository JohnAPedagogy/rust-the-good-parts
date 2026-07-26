fn main() {
    struct Rectangle { width: f64, height: f64 }
    impl Rectangle {
        fn area(&self) -> f64 { self.width * self.height }
        fn perimeter(&self) -> f64 { 2.0 * (self.width + self.height) }
    }
    let r = Rectangle { width: 10.0, height: 5.0 };
    println!("Area: {}", r.area());
    println!("Perimeter: {}", r.perimeter());
}
