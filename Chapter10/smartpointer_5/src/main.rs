enum A {
    B(i32),
    C(i32, Rc<A>),
}
use std::rc::Rc;
fn main() {
    let x = Rc::new(A::C(1, Rc::new(A::C(2, Rc::new(A::B(3))))));
    println!("count of x = {}", Rc::strong_count(&x));
    let _y = A::C(5, Rc::clone(&x));
    println!("count of x = {}", Rc::strong_count(&x));
    {
        let _z = A::C(10, Rc::clone(&x));
        println!("count of x = {}", Rc::strong_count(&x));
    }
    println!("count of x = {}", Rc::strong_count(&x));
}
