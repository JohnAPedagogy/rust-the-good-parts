fn main() {
    let b = Box::new(5);
    println!("b = {b}");
    #[derive(Debug)]
    enum List { Cons(i32, Box<List>), Nil }
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("{:?}", list);
}
