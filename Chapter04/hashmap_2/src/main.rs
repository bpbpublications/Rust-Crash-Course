use std::collections::HashMap;
fn main() {
    let mut currencies = HashMap::new();

    currencies.insert("India", "INR");
    currencies.insert("United States", "USD");
    currencies.insert("United Kingdom", "GBP");
    
    let currency_usa = currencies.get("United States");
    let currency_china = currencies.get("China");
    println!("Currency of USA is: {:?}", currency_usa);
    println!("Currency of China is: {:?}", currency_china);
}
