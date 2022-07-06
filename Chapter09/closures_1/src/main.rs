fn main() {
    fn add_nums_f(x:i32, y:i32) -> i32 {
        x + y
    }
    let add_nums_c = |x, y| {
        x + y
    };
    let a = add_nums_f(10, 15);
    println!("a = {}", a);
    
    let b = add_nums_c(10, 15);
    println!("b = {}", b);
}
