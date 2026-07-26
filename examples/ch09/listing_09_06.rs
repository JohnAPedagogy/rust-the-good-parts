fn main() {
    let name = String::from("Bob");
    let greet = move || println!("Hello, {name}!");
    greet();
}
