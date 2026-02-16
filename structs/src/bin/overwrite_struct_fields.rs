fn main() {
    // the struct fields do not care about imutability / mutability
    // but the struct itself does
    // whatever owns the struct, has to be either mutable / immutable
    struct Coffee {
        price: f64,
        name: String,
        is_hot: bool,
    }

    // making the variable that holds the struct mutable
    // allows overriding data
    let mut beverage: Coffee = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true,
    };

    // override happens here
    beverage.name = String::from("Caramel Macchiato");
    beverage.price = 6.99;
    beverage.is_hot = false;

    println!(
        "My {} this morning cost {}. It is {} that it was hot.",
        beverage.name, beverage.price, beverage.is_hot
    );
}
