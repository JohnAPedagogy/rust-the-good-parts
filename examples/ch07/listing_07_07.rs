fn main() {
    fn sqrt_of_string(s: &str) -> Result<f64, String> {
        s.trim()
            .parse::<f64>()
            .map_err(|e| format!("Could not parse '{}': {}", s.trim(), e))
            .and_then(|n| {
                if n < 0.0 {
                    Err(format!("Cannot take sqrt of negative number: {}", n))
                } else {
                    Ok(n.sqrt())
                }
            })
            .map(|result| (result * 1000.0).round() / 1000.0)
    }
    println!("{:?}", sqrt_of_string("9.0"));
    println!("{:?}", sqrt_of_string("abc"));
    println!("{:?}", sqrt_of_string("-4.0"));
}
