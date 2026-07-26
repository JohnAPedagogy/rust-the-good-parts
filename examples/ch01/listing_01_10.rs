fn main() {
    let inferred = 42;
    let explicit: u8 = 42;
    let also_explicit = 42u8;
    println!("{inferred} {explicit} {also_explicit}");
}
