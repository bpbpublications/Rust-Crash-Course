fn main() {
    let arr = [1, 2, 3, 4, 5];
    let arr_iter = arr.iter();  // iterator created
    for val in arr_iter {   // iterator used here
        print!("{} ", val);
    }
}