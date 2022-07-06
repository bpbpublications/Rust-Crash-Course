fn main() {
    let v = vec!(1, 2, 3, 4, 5);
    //let iter = v.iter();
    let iter = v.into_iter();
    for val in iter {
        println!("{} ", val);
    }
    println!("{:?}", v); // reusing the collection
}
