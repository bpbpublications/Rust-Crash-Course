fn main() {
    let num = 5;
    let add_num = |x| {
        x + num
    };
    let a = add_num(10);
    println!("a = {}", a);
}
