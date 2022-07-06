fn main() {
    let x = 10;
    let y = 20;
    let x_p = &x;
    
    println!("x = {}, &x = {:p}", x, &x);
    println!("y = {}, &y = {:p}", y, &y);
    println!("x_p = {:p}", x_p);
}