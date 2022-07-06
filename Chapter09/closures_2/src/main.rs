fn main() {
    let add_nums_c = |x, y| {
        x + y
    };
    let b = add_nums_c(10, 15); // First called with integers
    println!("b = {}", b);
    
    let c = add_nums_c(10.0, 15.0); // Called again with floats
    println!("c = {}", c);
}
