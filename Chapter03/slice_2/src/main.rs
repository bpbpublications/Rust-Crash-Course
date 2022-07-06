fn main() {
    let s = String::from("Gullu Seth");
    let w1 = get_slice(&s);
    println!("{}", w1);
}

fn get_slice(s : &String) -> &str {
    return &s[0..6];
}
