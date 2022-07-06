fn main() {
    let mut v = vec!(1, 2, 3, 4, 5);
    let iter = v.iter_mut();
    for val in iter {
        *val = *val+100;
    }
    println!("{:?}", v); // reusing the collection
}