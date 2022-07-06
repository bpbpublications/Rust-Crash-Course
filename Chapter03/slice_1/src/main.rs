fn main() {
    let s = String::from("Gullu Seth");
    let w1 = &s[0..5];
    let w2 = &s[6..10];
    println!("{}: {}", w1, w1.len());
    println!("{}: {}", w2, w2.len());
}
