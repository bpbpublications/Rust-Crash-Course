pub struct CustomStack {
    elements: Vec<i32>,
    max: i32,
}

impl CustomStack {
    pub fn push(&mut self, elem: i32) {
        self.elements.push(elem);
        self.calculate_max();
    }
    pub fn pop(&mut self) -> Option<i32> {
        let el = self.elements.pop();
        match el {
            Some(x) => {
                self.calculate_max();
                Some(x)
            }
            none => none,
        }
    }
    pub fn max(&self) -> i32 {
        self.max
    }
    fn calculate_max(&mut self) {
        // Hidden: Internal logic 
        let m = self.elements.iter().max();
        match m {
            None => self.max = 0,
            Some(x) => self.max = *x
        }
    }
}

fn main(){
    let mut cs = CustomStack{ elements: vec![], max: 0 };
    cs.push(5);
    cs.push(3);
    cs.push(10);
    println!("Max value in stack = {}",cs.max());
    cs.pop();
    println!("Max value in stack = {}",cs.max());
}
