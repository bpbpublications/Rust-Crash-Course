fn main() {
    let arr = [1, 2, 3, 4, 5];
    let mut arr_iter = arr.iter();  // iterator created
    
    println!("{}", arr_iter.next().unwrap());
    println!("{}", arr_iter.next().unwrap());
    println!("{}", arr_iter.next().unwrap());
    println!("{}", arr_iter.next().unwrap());
    println!("{}", arr_iter.next().unwrap());
  // println!("{}", arr_iter.next().unwrap()); // panic due to unwrap() on None value
}
