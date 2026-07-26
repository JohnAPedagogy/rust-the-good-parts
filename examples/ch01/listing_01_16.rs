fn main() {
    let owned: String = String::from("Hello");
    let borrowed: &str = "world";
    println!("{owned} {borrowed}");
}
