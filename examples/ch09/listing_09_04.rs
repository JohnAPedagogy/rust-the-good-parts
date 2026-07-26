fn main() {
    let name = String::from("Alice");
    let greet = || println!("Hello, {name}!");
    greet();
    println!("{name}");
    greet();
}
