use std::time::Instant;
// sum using for loop
fn sum_1(x: &[i64]) -> i64 {
    let mut result: i64 = 0;
    for i in 0..x.len() {
        result += x[i];
    }
    result
}
// sum using iterator
fn sum_2(x: &[i64]) -> i64 {
    x.iter().sum()
}

fn main() {
    let mut v = vec![0; 1000000];
    let mut count = 1;
    for i in 0..v.len() {
        v[i] = count;
        count += 1;
    }
    
    for i in 0..5 {
        println!("Run {} : ", i);
        let mut now = Instant::now();
        let sum1 = sum_1(&v);
        println!("sum_1: {} / {} ms", sum1, now.elapsed().as_millis());
        now = Instant::now();
        let sum2 = sum_2(&v);
        println!("sum_2: {} / {} ms", sum2, now.elapsed().as_millis());
    }
}
