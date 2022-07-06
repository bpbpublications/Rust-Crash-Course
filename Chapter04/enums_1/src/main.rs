#[derive(Debug)]
enum Location {
    India (String),
    US (String),
    UK (String)
}

#[derive(Debug)]
struct Employee {
    name: String,
    age: u8,
    email_id: String,
    experience: u8,
    location: Location
 }

fn main() {
    let _bangalore = Location::India(String::from("Bangalore"));
    let _delhi = Location::India(String::from("Delhi"));
    let _sanjose = Location::US(String::from("San Jose"));
    let _ny = Location::US(String::from("New York"));
    let _london = Location::UK(String::from("London"));

    let emp1 = Employee {
        name: String::from("Usha"),
        age: 55,
        email_id: String::from("usha@company.com"),
        experience: 25,
        location: _bangalore
    };
    println!("emp1: {:?}", emp1);
}
