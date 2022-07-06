fn main() {
    //let a:i32 = "10".parse().unwrap();
    //let a:i32 = "10a".parse().unwrap();
    let a:i32 = "10a".parse().expect("Oops invalid string.");
    println!("a = {}", a);
}
