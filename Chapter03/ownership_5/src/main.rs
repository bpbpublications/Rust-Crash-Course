fn main() {
  let s = String::from("rust");
  foo_string(s);
  //println!("{}", s); // ERROR: s is moved

  let x = 10;
  foo_int(x); 
  println!("{}", x);
} 

fn foo_string(ss: String) { // ss comes into scope
    println!("{}", ss);
} // ss goes out of scope and 'drop' is called & memory is freed.

fn foo_int(a: i32) {
    println!("{}", a);
}
rust
10
10