use std::rc::Rc;

fn main() {
    enum RcList { Cons(i32, Rc<RcList>), Nil }
    let a = Rc::new(RcList::Cons(5, Rc::new(RcList::Cons(10, Rc::new(RcList::Nil)))));
    let _b = RcList::Cons(3, Rc::clone(&a));
    let _c = RcList::Cons(4, Rc::clone(&a));
    println!("Reference count: {}", Rc::strong_count(&a));
}
