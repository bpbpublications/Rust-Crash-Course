fn main() {
    let sa = String::from("rust");
    println!("sa = {}", sa); // Fine here
    
    let sb = sa;  // sa assigned to sb here
    
    println!("sa = {}", sa);  // ERROR
    println!("sb = {}", sb);  // Ok
}
