use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx1, rx) = mpsc::channel();
    let tx2 = tx1.clone();
    
    thread::spawn(move || {
        let num_vec = vec![1, 2, 3];
        for num in num_vec {
            tx1.send(num).unwrap();
            thread::sleep(Duration::from_millis(2));
        }
    });
    
    thread::spawn(move || {
        let num_vec =
            vec![4, 5, 6];
        for num in num_vec {
            tx2.send(num).unwrap();
            thread::sleep(Duration::from_millis(2));
        }
    });
    
    for received_val in rx {
        println!("Received : {}",
            received_val);
    }
}
