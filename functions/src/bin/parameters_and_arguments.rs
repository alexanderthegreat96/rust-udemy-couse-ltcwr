fn main() {
    // a parameter is a name for an expected input
    // the argument is the concrete value passed into the function
    // parameters -> ex: fn print_name(name: String)
    // argument -> ex: print_name(String::from("alex"))

    // function takes a string literal
    // therefore, we do that
    // here we do that using a string literal
    // argument
    open_store("Brooklyn");
    bake_pizza(20, "pepperoni");
    swim_in_profit();
    swim_in_profit();
    swim_in_profit();
    open_store("Queens");
    bake_pizza(15, "mushroom");
}

// when defining parameters
// parameter_name: parameter_type
fn open_store(neighborhood: &str) {
    println!("Opening my pizza store in {neighborhood}");
}

fn bake_pizza(number: i32, topping: &str) {
    println!("Baking {number} {topping} pizzas");
}

fn swim_in_profit() {
    println!("So much $$$, so little time");
}
