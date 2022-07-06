#[allow(dead_code)]
enum Grade {
    A,
    B,
    C,
    D
}

fn main() {
    let grade = Grade::B;
    match grade {
        Grade::A => println!("Excellent"),
        Grade::B => println!("Very good"),
        Grade::C => println!("Good"),
        Grade::D => println!("Poor")
    }
}
