fn main() {
    let values: Vec<Option<i32>> = vec![Some(5), None, Some(10)];
    for v in &values {
        let n = v.unwrap_or_else(|| 0);
        println!("{n}");
    }
    let n: Option<i32> = None;
    println!("or: {}", n.unwrap_or(0));
}
