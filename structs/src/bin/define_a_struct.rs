fn main() {
    // similar to objects in oop
    // or classes
    // they combine multiple data types into 1
    struct Coffee {
        price: f64,
        name: String,
        is_hot: bool,
    }

    let mocha = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true,
    };

    println!(
        "My {} this morning cost {}. It is {} that it was hot.",
        mocha.name, mocha.price, mocha.is_hot
    );

    let favorite_coffee = mocha.name;
    println!("{favorite_coffee}");

    // println!("{}", mocha.name);
}
