fn main() {
    let mut v = vec![String::from("a"), String::from("b"), String::from("c")];
    let cloned = v[0].clone();
    println!("Cloned: {cloned}, vec: {v:?}");
    let removed = v.remove(0);
    println!("Removed: {removed}, vec: {v:?}");
}
