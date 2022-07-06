use std::sync::{Arc, Mutex};
use std::thread;
fn main() {
    let val = Arc::new(Mutex::new(10));
    let mut t_handles = vec![];
    println!("val = {}", *val.lock().unwrap());

    for _ in 0..5 {
        let val = val.clone();
        let h = thread::spawn(move || {
            let mut num = val.lock().unwrap();
            *num += 1;
        });
        t_handles.push(h);
    }
    for h in t_handles {
        h.join().unwrap();
    }
    println!("val = {}", *val.lock().unwrap());
}
