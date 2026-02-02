fn main() {
    // you can have multiple immutable references
    // but only 1 mutable one
    // multiple readers, but 1 writer
    let car = String::from("Red");
    let ref1 = &car;
    let ref2 = &car;
    println!("{ref1} and {ref2} and {}", &car);
}
