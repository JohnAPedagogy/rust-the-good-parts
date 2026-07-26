fn main() {
    fn multiply(x: i32, factor: i32) -> i32 { x * factor }
    let factor = 3;
    let multiply_by_factor = |x| x * factor;
    println!("{}", multiply(5, 3));
    println!("{}", multiply_by_factor(5));
}
