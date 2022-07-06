#[derive(Debug)]
struct Circle<T> {
    cx : T,
    cy : T,
    r : T
}

fn main() {
    let c1 = Circle {
        cx : 10,
        cy : 20,
        r : 5
    };
    println!("c1 = {:?}", c1);
}
