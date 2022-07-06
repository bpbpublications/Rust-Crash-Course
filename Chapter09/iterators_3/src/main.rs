struct CounterFinite {
    count: u32,
}

impl Iterator for CounterFinite {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 10 {
            self.count += 1;
            return Some(self.count);
        }
        None
    }
}

fn main() {
    let mut cf = CounterFinite {
        count: 0,
    };
    //for i in cf {
    //    println!("{}", i);
    //}

    println!("{}", cf.next().unwrap());
    println!("{}", cf.next().unwrap());
}
