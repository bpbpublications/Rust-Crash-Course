fn main() {
    let s1 = String::from("Rust");
    let s2 = String::from(" Programming");
    let s3 = s1 + &s2;
    println!("s3 = {}", s3);
    let s4 = s3 + " book"; 
    println!("s4 = {}", s4);
}
