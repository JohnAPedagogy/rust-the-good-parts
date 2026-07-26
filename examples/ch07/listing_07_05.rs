fn main() {
    fn read_username() -> Result<String, String> {
        let input = "alice\n";
        let first = input.lines().next().ok_or("Empty input")?;
        if first.is_empty() {
            Err("Username is empty".to_string())
        } else {
            Ok(first.to_string())
        }
    }
    println!("{:?}", read_username());
}
