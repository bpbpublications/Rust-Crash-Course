fn sum_int(x:i32, y:i32) -> i32 {
    x+y
}
fn main() {
    let a = 10;
    let b = 20;
    let c = sum_int(a, b);
    println!("{}", c);
}