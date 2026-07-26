fn main() {
    mod maths {
        pub fn square(x: i32) -> i32 {
            x * x
        }
    }
    let result = maths::square(5);
    println!("{result}");
}
