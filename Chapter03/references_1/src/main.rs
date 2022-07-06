fn main() {
    let s = String::from("rust");
    foo_string_ref(&s);  // 's' passed as reference
    println!("{}", s); // 's' is borrowed
} 
  
fn foo_string_ref(ss: &String) {  // &String : String reference
    println!("{}", ss);
} 

