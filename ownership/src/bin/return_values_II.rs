fn main() {
    let mut current_meal = String::new();
    current_meal = add_flour(current_meal);
    current_meal = add_sugar(current_meal);
    add_salt(&mut current_meal);
    println!("{}", current_meal);
}
// THIS IS A BAD APPROACH
// using a mutable reference is the way to go
// we're giving ownership back
// by sending it back
// it still needs to be saved to a new variable
fn add_flour(mut meal: String) -> String {
    meal.push_str("Add flour\n");
    meal
}

fn add_sugar(mut meal: String) -> String {
    meal.push_str("Add sugar\n");
    meal
}

// this is the way to go
fn add_salt(meal: &mut String) {
    meal.push_str("Added salt\n");
}
