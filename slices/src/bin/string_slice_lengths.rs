fn main() {
    // it's important to understand that when we are
    // slicing a collection
    // we're essentially slicing bytes
    // not all characters ocupy 1 byte
    // certain characters can occupy multiple
    // for example, the pizza slice emoji
    // since it's a unicode char. the length in bytes
    // is 4, although it looks like 1
    let food = "🍕";
    println!("{}", food.len());
    // if we are using 3 instead of 4 below
    // this will not compile, that's because
    // we cannot slice partial characters
    let pizza_slice = &food[0..4];
    println!("{}", pizza_slice.len());
}
