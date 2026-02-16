struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

fn main() {
    let mocha = make_coffee(String::from("Mocha"), 4.99, true);

    // weird syntax
    // but looks similar to slicing
    // so what happens here:
    // we can copy or use whatever data we want
    // we provide the named arguments from the struct
    // but when we do ..variable
    // anything else after "name", in this example
    // will be copied over
    let caramel_macchiato = Coffee {
        name: mocha.name.clone(), // we're copying here because string does not implement copy
        ..mocha                   // these are copied if they implement the copy trait
    };

    println!("{}", caramel_macchiato.name);
    println!("{}", mocha.name);
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot,
    }
}
