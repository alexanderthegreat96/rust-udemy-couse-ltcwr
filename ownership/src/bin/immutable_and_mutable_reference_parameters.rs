fn main() {
    let mut current_meal = String::new();
    add_flour(&mut current_meal);
    add_sugar(&mut current_meal);
    show_my_meal(&current_meal);
}

// meal: String
// mut meal: String
// meal: &String
// meal: &mut String

// this mutates data
fn add_flour(meal: &mut String) {
    meal.push_str("Add flour");
}

fn add_sugar(meal: &mut String) {
    meal.push_str(" Add Sugar");
}

// this will not allow you to modify
// mutate anything
// meal is the owner of the reference
// while current meal stays the owner
fn show_my_meal(meal: &String) {
    println!("Meal steps: {meal}");
}
