fn main() {
    let s1 = String::from("Rust");
    let s2 = String::from("Programming");
    let s3 = format!("{} {}", s1, s2);
    println!("s3 = {}", s3);
}
