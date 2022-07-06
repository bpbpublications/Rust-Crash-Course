use std::thread;
use std::time::Duration;

fn main() {
thread::spawn(|| {
for i in 1..10 {
println!("From spawned thread - {}", i);
thread::sleep(Duration::from_millis(2));
}
});
for i in 1..3 {
println!("From main thread - {}", i);
      	thread::sleep(Duration::from_millis(2));
}
}
