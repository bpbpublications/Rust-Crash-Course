fn main() {
    let mut x = 0;
    loop {
        x += 1;
        println!("x = {}", x);
        if x == 3 { break; }
    }

    // while loop
    let mut y = 0;
    while y < 3{
        y += 1;
        println!("y = {}", y);
    }

    // for loop
    let arr = [10, 20, 30, 40];
    for item in arr.iter(){
        println!("vsl = {}", item);
    }
}
