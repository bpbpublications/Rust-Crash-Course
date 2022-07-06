use std::ops::Mul;
fn area_rectangle<T:Mul<Output=T>>(length:T, width:T) -> T {
    length*width
}
fn main() {
    let l = 10;
    let w = 5;
    let area = area_rectangle(l, w);
    println!("area = {}", area);
}
