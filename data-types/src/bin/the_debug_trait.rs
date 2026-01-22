fn main() {
    // the debug trait
    // debug should format the output in a programmer-facing
    // debugging context
    // debug implements the display trait
    // we can inherit from a trait by using the derive macro

    let seasons = ["Spring", "Summer", "Fall", "Winter"];

    println!("{}", 5);
    println!("{}", 3.14);
    println!("{}", true);
    println!("{seasons:#?}"); // user friendly output -> pretty print
    println!("{seasons:?}"); // technical string representation
}
