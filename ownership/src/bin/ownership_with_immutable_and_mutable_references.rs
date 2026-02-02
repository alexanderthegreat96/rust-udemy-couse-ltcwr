fn main() {
    // an immutable reference implements the copy trait
    // that is because it is cheap

    let beans = String::from("beans");
    let d = &beans;
    // in here ownership doesn't shift
    let e = d;
    println!("{d} {e}");

    // an immutable reference does not implement the copy trait
    // the reason is simple, since you can only have 1 mutable reference
    // having 2 mutable references would point to the same spot
    let mut coffee = String::from("Mocha");
    let a = &mut coffee;

    println!("{a}");
    // in here ownership moves
    // therefore, you cannot print a anymore
    let b = a;
    println!("{b}");
}
