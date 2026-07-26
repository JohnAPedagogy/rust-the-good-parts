fn main() {
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list.iter() { if item > largest { largest = item; } }
        largest
    }
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest number: {}", largest(&numbers));
    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest char:   {}", largest(&chars));
}
