fn main() {
    fn largest_i32(list: &[i32]) -> &i32 {
        let mut largest = &list[0];
        for item in list { if item > largest { largest = item; } }
        largest
    }
    fn largest_f64(list: &[f64]) -> &f64 {
        let mut largest = &list[0];
        for item in list { if item > largest { largest = item; } }
        largest
    }
    println!("largest i32: {}", largest_i32(&[34, 50, 25, 100, 65]));
    println!("largest f64: {}", largest_f64(&[3.4, 5.0, 2.5, 10.0, 6.5]));
}
