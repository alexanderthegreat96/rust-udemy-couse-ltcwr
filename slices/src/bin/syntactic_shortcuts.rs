fn main() {
    let action_hero = String::from("Arnold Schwarzenegger");

    // if we are slicing from 0
    // we can remove the 0 from the square brackets
    // instead of 0..6 we simply do ..6
    let first_name = &action_hero[..6];
    println!("His first name is {first_name}.");

    // same here
    // if we want 7th byte all the way up to the end
    // we don't need to specify the last byte
    // this is specially useful when you don't know
    // the full length of the collection
    let last_name = &action_hero[7..];
    println!("His first name is {last_name}.");

    // this simply
    // grabs everything from the begging to the end
    // it creates a slice from the begging to the end
    // of the byte sequence
    let full_name = &action_hero[..];
    println!("His full name is {full_name}.");
}
