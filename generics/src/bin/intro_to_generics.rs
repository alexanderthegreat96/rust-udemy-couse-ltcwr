// the compiler will use the generic type
// to generate variants of the same code at compile time
// im using where keyword here to specify that T must IMPLEMENT Add
// which is a trait in the standard lib for additions

use std::ops::Add;
fn calculate<T>(input: T, output: T) -> T
where
    T: Add<Output = T>,
{
    return input + output;
}

// works with any type
// and returns it
fn identity<T>(value: T) -> T {
    value
}

fn main() {
    // a generic is a type argument
    // a generic abstract type that is a placeholder
    // for a future concrete type

    println!("{}", calculate(12.3, 10.0));
    println!("{}", calculate(10, 2));
    println!("{}", calculate(-10, 10));

    println!("{}", identity(10));
    println!("{}", identity(false));
    println!("{}", identity(String::from("mike")));
}
