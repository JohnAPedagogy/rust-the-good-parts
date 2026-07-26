fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}");

    let x = "out";
    {
        let x = "in";
        println!("{x}");
    }
    println!("{x}");

    let _draft = 42;
}
