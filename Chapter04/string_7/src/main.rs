fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("Привет");
    
    println!("{:?}", s1.chars());
    println!("{:?}", s1.bytes());
    println!("{:?}", s2.chars());
    println!("{:?}", s2.bytes());
}
