fn main() {
    let person = String::from("Boris");
    // this clones the entire
    // data
    // copies are avoided by default
    // but clone is basically copying instead of moving
    let genius = person.clone();

    println!("This is genius: {genius}");

    // this works because ownership never changed
    // since the stack + heap data was copied
    println!("This is {person}.");
}
