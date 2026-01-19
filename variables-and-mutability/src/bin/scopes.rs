fn main() {
    let coffee_price = 5.99;
    // this is a scope
    // this is isolated
    {
        let coffee_price = 1.99;
        println!("The coffee price is {coffee_price}"); // prints coffe_price from line 7
    }

    println!("The coffee price is {coffee_price}"); // prints coffee_price from line 3

    // scopes are connected to the idea of blocks
    // a block is the region between curly braces
    // {this is a block}
    {
        let name: &str = "mike";
        println!("hello {}", name);
    }

    let name: &str = "someone";
    println!("hello {}", name);

    // every function has it's scope
    // hence why the curly brackets
    // in this case, name goes out of scope
    // when exiting the main function
}
