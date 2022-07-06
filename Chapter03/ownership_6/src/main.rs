fn main() {
    let s = get_string();
    println!("{}", s);
}

fn get_string() -> String {
    let ss = String::from("rust");
    ss    // ss is moved
}

