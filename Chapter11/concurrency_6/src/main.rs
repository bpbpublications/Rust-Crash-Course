use std::sync::Mutex;
fn main() {
    let mu = Mutex::new(10); 
    println!("{:?}", mu);
    let mut val = mu.lock().unwrap();
    *val += 1;
    println!("{:?}", mu);
    std::mem::drop(val);
    println!("{:?}", mu);
}
