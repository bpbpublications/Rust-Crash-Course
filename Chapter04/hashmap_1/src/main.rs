use std::collections::HashMap;
fn main() {
    let mut currencies = HashMap::new();

    currencies.insert("India", "Rupees");
    currencies.insert("United States", "USD");
    currencies.insert("United Kingdom", "GBP");
    currencies.insert("India", "INR");
    
    println!("{:?}", currencies);
}
