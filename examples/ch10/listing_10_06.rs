use std::fmt;

fn main() {
    fn display_two<T, U>(a: &T, b: &U)
    where T: fmt::Display, U: fmt::Display {
        println!("{a} and {b}");
    }
    display_two(&42, &"hello");
    fn compare_and_display<T, U>(a: &T, b: &U)
    where T: fmt::Display + PartialOrd, U: fmt::Display {
        println!("{a} and {b}");
    }
    compare_and_display(&10, &5);
}
