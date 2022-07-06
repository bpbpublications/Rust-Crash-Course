fn main() {
    let mut v1: Vec<u8> = Vec::new();
    let mut v2 = vec![1.5, 2.5, 3.5];
    println!("v1 = {:?}, v2 = {:?}", v1, v2);
        
    v1.push(10);
    v1.push(20);
    v1.push(100);
    v2.pop();
    println!("v1 = {:?}, v2 = {:?}", v1, v2);
}
    