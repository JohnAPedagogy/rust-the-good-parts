fn main() {
    let number = 4;
    let description = match number {
        1 | 2 => "small",
        3..=5 => "medium",
        _ => "large",
    };
    println!("{number} is {description}");
}
