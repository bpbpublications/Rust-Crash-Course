struct Employee {
    name: String,
    age: u8,
    email_id: String,
    experience: u8,
 }
 #[allow(dead_code)]
impl Employee{
    // Employee fields can be accessed through 'self' using dot operator
    fn get_name(&self) -> &str {
       &self.name
    }
    fn set_age(&mut self, age: u8) {
        self.age = age;
    }
    fn print_employee(&self) {
        println!("Employee: name = {}, age = {}, email_id = {}, experience = {}",
        self.name, self.age, self.email_id, self.experience);
    }
 }
 impl Employee {
    fn new(name: String) -> Employee {
         Employee {
              name: name,
              age: 25,
              email_id: String::from("abc@company.com"),
              experience: 0
         }
    }
}
fn main() {
    let emp2 = Employee::new(String::from("Abhishek"));
    emp2.print_employee();
}
