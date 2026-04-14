fn main() {
    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let pizza_toppings = vec![pepperoni, mushroom, sausage];
    
    // transfers ownership to delicious_toppings
    let mut delicious_toppings = pizza_toppings;

    // here it does not transfer ownership
    // same ownership rules
    // mutliple immutable references are okay
    // but only one mutable reference at a time
    let topping_reference = &delicious_toppings[1]; // these 2 are file
    let _another_reference = &delicious_toppings[1]; // same here
    println!("The topping is {topping_reference}");

    delicious_toppings.push(String::from("Olives"));
    //println!("{:?}", _another_reference); // won't work, beacuse that mutable reference is already
    // in use
    // therefore we need another ref to make this work
    // this prevents overriding the data by mistake at any point
}
