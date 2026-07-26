fn main() {
    fn greet(name: &str) {
        println!("Hello, {name}!");
    }
    greet("Iyalla");
    let result = greet("world");
    println!("{:?}", result);
}
