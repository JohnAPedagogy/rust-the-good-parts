fn main() {
    #[derive(Debug, PartialEq, Eq)]
    struct Colour { r: u8, g: u8, b: u8 }
    let red = Colour { r: 255, g: 0, b: 0 };
    let also_red = Colour { r: 255, g: 0, b: 0 };
    println!("{}", red == also_red);
}
