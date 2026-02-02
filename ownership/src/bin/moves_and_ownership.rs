fn main() {
    // 1. DATA ALLOCATION
    // "person" is the owner.
    // Metadata (pointer, len, cap) is on the STACK.
    // The string "Boris" is on the HEAP.
    let person = String::from("Boris");
    println!("My name is {person}");

    // 2. THE MOVE
    // Ownership is transferred from "person" to "genius".
    // Rust copies the STACK data (the pointer) to "genius".
    // Crucially, "person" is now invalidated (it can no longer be used).
    let genius = person;

    // 3. THE HEAP REMAINS
    // The actual data "Boris" has not moved in memory;
    // it still lives at the same HEAP address.
    // Only the 'key' (the pointer) changed hands.

    // 4. COMPILER CHECK
    // This fails because "person" is no longer a valid owner.
    // println!("My name is {person}");
    println!("The genius is {genius}");

    // Important:
    // this feature solves the double free error
} // 5. CLEANUP
// When "genius" goes out of scope, the HEAP memory is freed.
// Because "person" was moved, Rust knows NOT to try and free it again.
