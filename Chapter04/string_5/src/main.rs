fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("Привет");
    let len1 = s1.len();
    let len2 = s2.len();
    println!("len1 = {}, len2 = {}", len1, len2);
    println!("{}", s1[0]);
}
