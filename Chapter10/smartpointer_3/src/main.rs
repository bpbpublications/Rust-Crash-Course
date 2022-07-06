enum A {
    B(i32),
    C(i32, A),
}
fn main() {
    let a = A::C(1, A::C(2, A::B(3)));
}
