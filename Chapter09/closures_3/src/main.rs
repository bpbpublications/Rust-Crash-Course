struct Circle<T>
where T: Fn(f32) -> f32 {
    radius : f32,
    area : T,
}

fn main() {
    let pi = 3.14;
    let area_circle = |r| {
        pi*r*r
    };
    let c = Circle{
        area: area_circle,
        radius: 10.0,
    };
    println!("area of circle = {}", (c.area)(c.radius));
}
