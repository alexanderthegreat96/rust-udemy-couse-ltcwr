#[derive(Debug)]
struct Coffee {
    name: String,
    price: f64,
    is_hot: bool,
}

fn main() {
    // as a general idea
    // structs do not implement Display or Debug traits
    let mocha: Coffee = make_coffee(String::from("Mocha"), 4.99, true);

    println!("{:?}", mocha); // regular
    println!("{:#?}", mocha); // debug
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot,
    }
}

#[allow(dead_code)]
fn drink_coffee(coffee: &mut Coffee) {
    println!("Drinking my delicious {}", coffee.name);
    coffee.is_hot = false;
    coffee.price = 10.99;
}
