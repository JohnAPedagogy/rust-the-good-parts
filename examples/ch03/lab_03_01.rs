fn main() {
    mod conversions {
        pub fn celsius_to_fahrenheit(c: f64) -> f64 {
            c * 9.0 / 5.0 + 32.0
        }
        pub fn fahrenheit_to_celsius(f: f64) -> f64 {
            (f - 32.0) * 5.0 / 9.0
        }
        pub fn celsius_to_kelvin(c: f64) -> f64 {
            c + 273.15
        }
    }
    let c = 25.0;
    let f = conversions::celsius_to_fahrenheit(c);
    let k = conversions::celsius_to_kelvin(c);
    println!("{c:.2} °C = {f:.2} °F = {k:.2} K");
}
