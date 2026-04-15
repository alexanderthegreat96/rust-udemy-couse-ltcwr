use std::collections::HashMap;

fn main() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    coffee_pairings.insert(&drink, &milk);
    coffee_pairings.insert("Flat White", "Almond Milk");

    // basically, the key is the entrypoint
    // so we can use entry where the key is somehting or insert
    coffee_pairings.entry("Latte").or_insert("Pistachio Milk");
    println!("{coffee_pairings:?}");

    // basically, if the key exists it will not insert
    // so it will inseert a key value pair ONLY IF the key
    // DOES NOT EXIST
    coffee_pairings
        .entry("Cappuccino")
        .or_insert("Pistachio Milk");
    println!("{coffee_pairings:?}");
}
