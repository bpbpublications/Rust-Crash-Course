use std::collections::HashMap;
use std::io::Read;
 
struct Todo {
    map: HashMap<String, String>,
}
 
impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("todo.db")?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let map: HashMap<String, String> = content
            .lines()
            .map(|line| line.split(" : ").collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), String::from(v)))
            .collect();
        Ok(Todo { map })
    }
 
    fn insert(&mut self, key: String) {
        self.map.insert(key, String::from("To Do"));
    }
 
    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{} : {}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("todo.db", content)
    }

    fn start(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = String::from("In Progress")),
            None => None,
        }
    }
 
    fn done(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = String::from("Done")),
            None => None,
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        panic!("Too less arguments passed...");
    }
    let action = args[1].clone(); //std::env::args().nth(1).unwrap();
    let task = args[2].clone(); //std::env::args().nth(2).unwrap();
 
    let mut todo = Todo::new().expect("Error in ToDo list creation");
 
    if action == "add" {
        todo.insert(task);
        match todo.save() {
            Ok(_) => println!("Task added"),
            Err(e) => println!("Error : {}", e),
        }
    }
    else if action == "start" {
        match todo.start(&task) {
            None => println!("'{}' not present in ToDo list", task),
            Some(_) => match todo.save() {
                Ok(_) => println!("Task started"),
                Err(e) => println!("Error : {}", e),
            },
        }
    }
    else if action == "done" {
        match todo.done(&task) {
            None => println!("'{}' not present in ToDo list", task),
            Some(_) => match todo.save() {
                Ok(_) => println!("Task Done"),
                Err(e) => println!("Error : {}", e),
            },
        }
    }
}
