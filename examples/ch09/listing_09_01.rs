fn main() {
    let add = |a, b| a + b;
    let square = |x: i32| -> i32 { x * x };
    println!("add(3, 4): {}", add(3, 4));
    println!("square(5): {}", square(5));
}
