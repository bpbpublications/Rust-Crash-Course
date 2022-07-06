fn area_square(side: &str) -> i32 {
    let x:i32 = side.parse().unwrap();
    x*x
}

fn main() {
    //let area = area_square("10");
    let area = area_square("10a");
    println!("area = {}", area);
}
