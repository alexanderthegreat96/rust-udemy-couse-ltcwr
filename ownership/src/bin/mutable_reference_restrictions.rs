fn main() {
    // you cannot have multiple mutable
    // references to the same value
    let mut car = String::from("Red");
    let ref1 = &mut car; // the lifetime of this ends at line 3
    let ref2 = &car;
    // this code compiles fine because
    // ref1 doesn't actually mutate the car variable

    // this is basically a lifetime
    // esentially, the reference cannot outlive whatever it points to
    // this prevents dangling pointers basically
    println!("{ref2}");
}
