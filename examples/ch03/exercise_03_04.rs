fn main() {
    fn read_guess(min: u32, max: u32) -> Option<u32> {
        match "5".trim().parse() {
            Ok(value) => {
                if value >= min && value <= max {
                    Some(value)
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
    println!("{:?}", read_guess(1, 10));
    println!("{:?}", read_guess(10, 20));
}
