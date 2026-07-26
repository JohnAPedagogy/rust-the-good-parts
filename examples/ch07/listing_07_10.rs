use std::fmt;

fn main() {
    #[derive(Debug)]
    enum AppError {
        IoError(String),
        ParseError(String),
        InvalidInput(String),
    }
    impl fmt::Display for AppError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                AppError::IoError(e) => write!(f, "IO error: {e}"),
                AppError::ParseError(e) => write!(f, "Parse error: {e}"),
                AppError::InvalidInput(s) => write!(f, "Invalid input: {s}"),
            }
        }
    }
    impl std::error::Error for AppError {}
    fn load_count(s: &str) -> Result<i32, AppError> {
        s.parse::<i32>()
            .map_err(|e| AppError::ParseError(e.to_string()))
            .and_then(|n| {
                if n < 0 {
                    Err(AppError::InvalidInput(format!("negative: {n}")))
                } else {
                    Ok(n)
                }
            })
    }
    match load_count("42") {
        Ok(n) => println!("Count: {n}"),
        Err(e) => println!("Error: {e}"),
    }
    match load_count("-5") {
        Ok(n) => println!("Count: {n}"),
        Err(e) => println!("Error: {e}"),
    }
}
