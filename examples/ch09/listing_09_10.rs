fn main() {
    fn make_handler(flag: bool) -> Box<dyn Fn(i32) -> i32> {
        if flag { Box::new(|x| x * 2) } else { Box::new(|x| x + 1) }
    }
    println!("handler(true, 5): {}", make_handler(true)(5));
    println!("handler(false, 5): {}", make_handler(false)(5));
}
