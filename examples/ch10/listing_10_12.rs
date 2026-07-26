use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    *data.borrow_mut() += 1;
    println!("data = {:?}", data.borrow());
}
