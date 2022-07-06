struct Employee {
    name: String,
    age: u8,
    email_id: String,
    experience: u8,
 }

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

fn main() {
    let mut emp1 = Employee {
        name: String::from("Harshita"),
        age: 26,
        email_id: String::from("harshita@company.com"),
        experience: 1
    };
    println!("Employee name is: {}", emp1.get_name());
    emp1.set_age(27);
    emp1.print_employee();
 }