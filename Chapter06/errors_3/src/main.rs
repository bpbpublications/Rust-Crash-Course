use std::num::ParseIntError;

fn area_square(side: &str) -> i32 {
    let x:Result<i32, ParseIntError> = side.parse();
    let x = match x {
        Ok(l) => l,
        Err(e) => panic!("Error occurred: {}", e),
    };
    x*x
}


fn main() {
    let area = area_square("10a");
    println!("area = {}", area);
}
