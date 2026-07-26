fn main() {
    let point = (3, -1);
    match point {
        (0, 0) => println!("Origin"),
        (x, 0) => println!("On the x-axis at {x}"),
        (0, y) => println!("On the y-axis at {y}"),
        (x, y) => println!("At ({x}, {y})"),
    }
}
