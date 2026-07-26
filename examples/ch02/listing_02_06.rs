fn main() {
    let maybe_number: Option<i32> = Some(42);
    match maybe_number {
        Some(n) => println!("Got a number: {n}"),
        None => println!("Got nothing"),
    }
}
