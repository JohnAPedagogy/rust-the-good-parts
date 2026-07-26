fn main() {
    fn might_fail(n: i32) -> i32 {
        if n < 0 { panic!("negative number: {n}"); }
        if n == 0 { return 0; }
        if n > 100 { unreachable!("we never call this with n > 100"); }
        n * 2
    }
    let result = std::panic::catch_unwind(|| might_fail(-1));
    println!("Panic caught: {:?}", result);
    println!("normal: {}", might_fail(42));
}
