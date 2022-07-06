fn main() {
    let x: i32 = -10; 
    //let x: u32 = -10; // Error

    let y: f32 = 5.0;

    let f: bool = false;

    let c1 = 'a';
    let c2 = '5';
    let c3 = '\u{263A}';
    println!("c1 = {}, c2 = {}, c3 = {}", c1, c2, c3);

    let tup = (10, 'a', 10.5);
    let first_elem =tup.0;
    println!("{}", first_elem);

    let arr = [10, 20, 30];
    let num1 = arr[0];
    println!("num1 = {}", num1);

    // This is a single-line comment

    /* This is
     a multi-line
     comment */
}
