fn main() {
    fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }
    fn apply_twice<F: FnMut()>(mut f: F) { f(); f(); }
    fn apply_once<F: FnOnce() -> String>(f: F) -> String { f() }
    let result = apply(|x| x * 3, 10);
    println!("apply: {result}");
    let mut count = 0;
    apply_twice(|| count += 1);
    println!("count after apply_twice: {count}");
    let name = String::from("Alice");
    let greeting = apply_once(move || format!("Hello, {name}!"));
    println!("{greeting}");
}
