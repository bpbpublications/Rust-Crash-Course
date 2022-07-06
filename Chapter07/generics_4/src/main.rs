fn area_rect<T>(length:T, width:T) -> T {
    length*width
}

fn main() {
    let l = 10;
    let w = 5;
    let area = area_rect(l, w);
    println!("area = {}", area);
}
