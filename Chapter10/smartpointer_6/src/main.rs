#[derive(Debug)]
enum A {
    B(i32),
    C(Rc<RefCell<i32>>, Rc<A>),
}

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let x = Rc::new(RefCell::new(10));
    let y = Rc::new(A::C(Rc::clone(&x), Rc::new(A::B(1))));
    println!("y = {:?}", y);
    *x.borrow_mut() += 100;
    println!("y mutated = {:?}", y);
}
