use std::fmt;

fn main() {
    #[derive(Debug)]
    enum GuessError {
        Io(String),
        Parse { input: String, source: String },
    }
    impl fmt::Display for GuessError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                GuessError::Io(e) => write!(f, "IO: {e}"),
                GuessError::Parse { input, source } => write!(f, "'{input}' not a number: {source}"),
            }
        }
    }
    impl std::error::Error for GuessError {}
    fn read_guess(input: &str) -> Result<u32, GuessError> {
        input.trim().parse::<u32>().map_err(|e| GuessError::Parse {
            input: input.to_string(),
            source: e.to_string(),
        })
    }
    match read_guess("42") {
        Ok(n) => println!("Guess: {n}"),
        Err(e) => println!("Error: {e}"),
    }
    match read_guess("abc") {
        Ok(n) => println!("Guess: {n}"),
        Err(e) => println!("Error: {e}"),
    }
}
