#[allow(dead_code)]

mod cook_food;
use crate::cook_food::indian::cook_samosa;
use crate::cook_food::chinese::cook_noodles;
use crate::cook_food::italian::{cook_pizza, cook_pasta};
use crate::cook_food::mexican;

fn main() {
    println!("From main...");
    cook_samosa();
    cook_noodles();
    cook_pizza();
    cook_pasta();
    mexican::cook_burritos();
    mexican::cook_tomato_salsa();
}
