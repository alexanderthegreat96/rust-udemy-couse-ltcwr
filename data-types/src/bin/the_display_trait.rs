fn main() {
    // short intro to traits
    // traits are basically interfaces in other languages
    // they define behavior for code
    // the catch is, they can also define implementations
    // which can be used as they are or overriden
    // they are very powerul
    // the idea is that if something implements that trait
    // it must have thoes methods available
    // sort of like a real life contract

    // a type is not obligated to implement any given trait
    // a type can implement multiple traits

    let seasons = ["Spring", "Summer", "Fall", "Winter"];

    // primitive types all implement this trait
    // hence why we can print them without using :? inside {}
    println!("{}", 5);
    println!("{}", 3.14);
    println!("{}", true);

    // Invalid
    // `[&str; 4]` doesn't implement `std::fmt::Display`
    // the trait `std::fmt::Display` is not implemented for `[&str; 4]`
    // in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    // println!("{}", seasons);
    println!("{:?}", seasons);

    // the display trait
    // is a trait that requires that a type can be represented as a user friendly
    // readable string
    // the display trait mandates a format method that returns the string
}
