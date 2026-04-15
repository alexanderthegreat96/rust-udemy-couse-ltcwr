use std::collections::HashMap;

fn main() {
    // as with almost everything in rust
    // when something does not implement the copy trait
    // overship moves
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");

    // we can store string references
    coffee_pairings.insert(&drink, &milk);
    coffee_pairings.insert("Flat White", "Almond Milk");
    coffee_pairings.insert("Another", "Drink");

    println!("{}", coffee_pairings.len());
    println!("{drink} {milk}");
}
