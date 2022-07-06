use std::num::ParseIntError;

fn main() {
    let a:Result<i32, ParseIntError> = "10".parse();
    let b = a.is_ok();
    let c = a.is_err();
    println!("b = {}, c = {}", b, c);
}
