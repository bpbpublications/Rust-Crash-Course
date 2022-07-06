enum A {
    B(i32),
    C(i32, Box<A>),
}

fn main() {
    let x = A::C(1, Box::new(A::C(2, Box::new(A::B(3)))));
    let sz = std::mem::size_of_val(&x);
    println!("size of x = {}", sz);
}
