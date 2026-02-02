fn main() {
    // a dangling reference
    // is a pointer to a memory address
    // that has been deallocated
    let city = create_city();
    println!("{city}");
}

fn create_city() -> String {
    String::from("New York")
}

// this is a dangling reference
// a reference to data that will no longer exist
// this is because state becomes the owner
// deallocation happens when state goes out of scope
// at the end of the function block
// fn create_state() -> &String {
//     let state = String::from("Minessota");
//     return &state;
// }
