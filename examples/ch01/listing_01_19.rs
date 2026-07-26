fn main() {
    let point = (3, 4.0);
    println!("x = {}, y = {}", point.0, point.1);
    let (x, y) = point;
    println!("Destructured: x={x}, y={y}");
}
