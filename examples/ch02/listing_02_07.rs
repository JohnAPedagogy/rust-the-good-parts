fn main() {
    let maybe_number: Option<i32> = Some(42);
    if let Some(n) = maybe_number {
        println!("The number is {n}");
    }
}
