fn main() {
    fn celsius_to_fahrenheit(c: f64) -> f64 {
        c * 9.0 / 5.0 + 32.0
    }
    let samples = [0.0, 100.0, -40.0, 37.0];
    for c in samples {
        println!("{c:.1} °C = {:.1} °F", celsius_to_fahrenheit(c));
    }
}
