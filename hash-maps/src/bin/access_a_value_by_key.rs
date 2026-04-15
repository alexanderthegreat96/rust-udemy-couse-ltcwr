use std::collections::HashMap;

fn main() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");

    coffee_pairings.insert(&drink, &milk);
    coffee_pairings.insert("Flat White", "Almond Milk");
    coffee_pairings.insert("Big Black", "Dark Chocolate");

    let value = coffee_pairings
        .get("Flat White")
        .copied()
        .unwrap_or("Unknown Milk");

    println!("{value}");

    // this might panic btw
    let another_value = coffee_pairings["Big Black"];
    println!("{}", another_value);

    // the smarter way to access things and get data
    // we wanna copy it basically
    // so we don't move ownership basically
    // in this case, it's fine, but, in the case of a heap string
    // this is what happens

    let some_value = coffee_pairings
        .get("Big Black")
        .copied()
        .unwrap_or("Cannot find data");

    println!("{}", some_value);
}
