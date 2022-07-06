fn main() {
    let v = vec![1.5, 2.5, 3.5]; // v has 3 elements
    println!("Second element = {}", v[1]);
    println!("Second element = {:?}", v.get(1));
    println!("Fourth element = {:?}", v.get(3));
}
