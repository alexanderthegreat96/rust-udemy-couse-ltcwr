struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

fn main() {
    // common practice to match parameter name with variable name
    // for structs
    let name = String::from("Latte");
    let coffee: Coffee = make_coffee(name, 4.99, true);
    println!(
        "My {} this morning cost {}. It is {} that it was hot.",
        coffee.name, coffee.price, coffee.is_hot
    );

    let name = String::from("Latte");
    let price = 3.99;
    let is_hot = false;

    // we can simply provide the variables in order
    // as long as they match the name arguments with the name field
    #[allow(unused_variables)]
    let latte = Coffee {
        name,
        price,
        is_hot,
    };
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot,
    }
}
