use std::fmt::Write;

fn main() {
    let burger = String::from("Burger");
    add_fries(burger);

    let mut meal: String = String::from("Pizza with:\n");
    add_toppings(&mut meal, "peperoni");
    add_toppings(&mut meal, "mushrooms");
    add_toppings(&mut meal, "peppers");
    add_toppings(&mut meal, "julapenos");

    println!("{}", meal);
}

// we're tranfering ownership
// and we're allowing the owner
// which is the meal parameter
// to allow it to modify it
// but, since the ownership is moved
// burger goes out of scope after the function invokes it
fn add_fries(mut meal: String) {
    meal.push_str(" and Fries");
    println!("{meal}");
}

// to allow mutation of the original value
// we need a &mut Type refference
// writting directly into a created buffer
// using write
fn add_toppings(meal: &mut String, topping: &str) -> () {
    let _ = write!(meal, " - {}\n", topping);
}
