use std::collections::HashMap;
fn main() {
    let mut currencies = HashMap::new();

    currencies.insert("India", "INR");
    currencies.insert("United States", "USD");
    currencies.insert("United Kingdom", "GBP");
    
    for (key, value) in &currencies {
        println!("{} : {}", key, value);
    }
}
