fn main() {
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }
    fn repeat(text: &str, times: u32) -> String {
        text.repeat(times as usize)
    }
    println!("{}", is_even(42));
    println!("{}", repeat("ha", 3));
}
