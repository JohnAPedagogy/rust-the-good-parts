fn main() {
    mod greetings {
        pub fn english(name: &str) {
            println!("Hello, {name}!");
        }
        pub fn spanish(name: &str) {
            println!("\u{00A1}Hola, {name}!");
        }
    }
    greetings::english("Iyalla");
    greetings::spanish("Iyalla");
}
