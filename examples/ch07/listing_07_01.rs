fn main() {
    fn parse_number(s: &str) -> Result<i32, String> {
        match s.parse::<i32>() {
            Ok(n) => Ok(n),
            Err(e) => Err(format!("parse error: {e}")),
        }
    }
    match parse_number("42") {
        Ok(n) => println!("Got: {n}"),
        Err(e) => println!("Error: {e}"),
    }
}
