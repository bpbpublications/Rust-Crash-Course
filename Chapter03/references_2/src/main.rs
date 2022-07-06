fn main() {
    let mut s = String::from("rust"); // s is mutable
    change_string(&mut s);     // pass mutable reference
    println!("{}", s); 
} 
  
fn change_string(ss: &mut String) {  // accept mutable reference
    ss.push_str(" programming");
} 

