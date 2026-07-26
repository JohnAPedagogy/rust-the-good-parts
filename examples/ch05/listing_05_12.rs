use std::fmt;

fn main() {
    struct Rect { width: f64, height: f64 }
    impl fmt::Display for Rect {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Rect({}w x {}h)", self.width, self.height)
        }
    }
    let r = Rect { width: 10.0, height: 5.0 };
    println!("{}", r);
}
