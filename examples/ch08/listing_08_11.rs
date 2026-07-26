fn main() {
    struct Celsius(f64);
    struct Fahrenheit(f64);
    impl From<Celsius> for Fahrenheit {
        fn from(c: Celsius) -> Fahrenheit { Fahrenheit(c.0 * 9.0 / 5.0 + 32.0) }
    }
    let f: Fahrenheit = Celsius(100.0).into();
    println!("Boiling: {:?} F", f.0);
}
