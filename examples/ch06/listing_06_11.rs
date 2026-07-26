fn main() {
    let hello = String::from("Héllo");
    println!("safe slice: {}", &hello[0..1]);
    for ch in hello.chars() { print!("{ch} "); }
    println!();
}
