fn main() {
    fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x + n
    }
    let add5 = make_adder(5);
    println!("add5(3): {}", add5(3));
}
