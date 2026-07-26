fn main() {
    fn describe_parse_error(s: &str) -> String {
        match s.parse::<i32>() {
            Ok(_) => "Valid number".to_string(),
            Err(e) => match e.kind() {
                std::num::IntErrorKind::Empty => "Input was empty".to_string(),
                std::num::IntErrorKind::InvalidDigit => "Contains non-digits".to_string(),
                std::num::IntErrorKind::PosOverflow => "Too large".to_string(),
                std::num::IntErrorKind::NegOverflow => "Too negative".to_string(),
                _ => format!("Parse error: {e}"),
            },
        }
    }
    println!("'42': {}", describe_parse_error("42"));
    println!("'abc': {}", describe_parse_error("abc"));
    println!("'': {}", describe_parse_error(""));
}
