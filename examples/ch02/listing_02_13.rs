fn main() {
    for i in 0..10 {
        if i % 2 == 0 {
            continue;
        }
        if i == 7 {
            break;
        }
        println!("{i}");
    }
}
