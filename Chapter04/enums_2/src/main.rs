fn is_odd(num:i8) -> Option<bool> {
    if num % 2 == 1 {
        return Some(true);
    } else {
        return None;
    }
}

fn main() {
    println!("{:?}", is_odd(10));
    println!("{:?}", is_odd(11));
}
