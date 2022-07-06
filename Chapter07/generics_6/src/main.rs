#[allow(dead_code)]
struct Circle<T> {
    cx : T,
    cy : T,
    r : T
}

impl<T> Circle<T> {
    fn radius(&self) -> &T {
        &self.r
    }
}

fn main() {
    let c = Circle {
        cx : 10,
        cy : 20,
        r : 5
    };
    println!("radius of c = {}", c.radius());
}
