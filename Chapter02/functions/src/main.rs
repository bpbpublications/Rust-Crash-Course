fn main() {
    first_func();
    second_func();

    let sum = sum(10, 20);
    println!("Sum of 10 and 20 = {}", sum);
}

// first_func: No parameters
fn first_func(){
    println!("first_func called...");
}

// second_func: two parameters of type i32 and u8
fn second_func(x : i32, y : u8){
    println!("second_func called... {}, {}", x, y);
}

fn sum(a : i32, b : i32) -> i32 {
    return a + b;
}