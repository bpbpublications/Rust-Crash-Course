pub fn hash(s: &String) -> usize {
    let mut result: usize = 5381;
    for c in s.bytes() {
        result = result*33 + c as usize;
    }
    result
}

#[derive(Default, Clone)]
struct HashEntry {
    key: String,
    value: i32,
    in_use: bool,
}

pub struct HashTable {
    table: Vec<HashEntry>,
}

const TABLE_SIZE: usize = 10000;
impl HashTable {
    pub fn new() -> Self {
        Self {
            table: vec![HashEntry::default(); TABLE_SIZE],
        }
    }
    pub fn insert(&mut self, key: String, new_value: i32) {
        if let Some(index) = self.get_index(&key) {
            self.table[index].value = new_value;
        } else {
            let mut index = hash(&key) % TABLE_SIZE;
            while self.table[index].in_use {
                index = (index + 1) % TABLE_SIZE;
            }
            self.table[index].in_use = true;
            self.table[index].key = key;
            self.table[index].value = new_value;
        }
    }
    fn get_index(&self, key: &String) -> Option<usize> {
        let mut index = hash(&key) % TABLE_SIZE;
        for _ in 0..TABLE_SIZE {
            if !self.table[index].in_use {
                break;
            }
            if self.table[index].key == *key {
                break;
            }
            index = (index + 1) % TABLE_SIZE;
        }
        if self.table[index].in_use && self.table[index].key == *key {
            Some(index)
        } else {
            None
        }
    }
    pub fn get(&self, key: &String) -> Option<&i32> {
        if let Some(index) = self.get_index(key) {
            Some(&self.table[index].value)
        } else {
            None
        }
    }
}

fn main() {
    let mut hash = HashTable::new();
    hash.insert(String::from("abhishek"), 1_000_000);
    hash.insert(String::from("vijay"), 2_000_000);
    hash.insert(String::from("abhishek"), 1_500_000);
    
    let x:String = String::from("abhishek");
    println!("value for {} = {}", x, hash.get(&x).unwrap());
}
