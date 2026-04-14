fn main() {
    // the get method allows you to basically extract a value from a vector
    // using the index
    // it returns an Option of type T
    //
    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let pizza_toppings = vec![pepperoni, mushroom, sausage];

    let option = pizza_toppings.get(50);

    match option {
        Some(topping) => println!("The topping is {topping}"),
        None => println!("No value at that index position"),
    }

    // as you can see, it returns a reference to that item at that
    // index position
    if let Some(topping) = pizza_toppings.get(0) {
        if topping == "Pepperoni" {
            println!("We chose pepperoni pizza!");
        } 
    }


    let optional = pizza_toppings.get(3);
    match optional {
        Some(topping) => {
            println!("This is the topping: {:?}", topping);
        },
        None => {
            println!("Unable to find anything!");
        }
    }
}
