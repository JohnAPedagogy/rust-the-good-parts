fn main() {
    fn double(x: i32) -> i32 {
        x * 2
    }
    fn describe(n: i32) -> &'static str {
        if n < 0 {
            return "negative";
        }
        "non-negative"
    }
    println!("{}", double(5));
    println!("{}", describe(-3));
    println!("{}", describe(7));
}
