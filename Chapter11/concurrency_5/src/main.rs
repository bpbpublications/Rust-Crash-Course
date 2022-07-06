use std::sync::Mutex;
fn main() {
    let mu = Mutex::new(10);
    {
        let mut val = mu.lock().unwrap();
        println!("{:?}", mu);
        *val += 1;
    }
    println!("{:?}", mu);
}
