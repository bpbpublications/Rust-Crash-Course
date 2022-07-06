struct CounterInfinite {
    count: u32,
}

impl Iterator for CounterInfinite {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        return Some(self.count);
    }
}

fn main() {
    let mut ci = CounterInfinite {
        count: 0,
    };
    println!("{}", ci.next().unwrap());
    println!("{}", ci.next().unwrap());
}
