fn main() {
    'outer: for x in 0..5 {
        for y in 0..5 {
            if x + y == 6 {
                println!("Breaking at x={x}, y={y}");
                break 'outer;
            }
        }
    }
    println!("Done");
}
