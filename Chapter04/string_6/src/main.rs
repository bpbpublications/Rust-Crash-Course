fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("Привет");
    
    println!("{:?}", &s1[0..1]);
    println!("{:?}", &s2[0..4]);
}
