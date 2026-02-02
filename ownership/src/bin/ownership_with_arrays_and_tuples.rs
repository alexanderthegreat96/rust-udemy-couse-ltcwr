fn main() {
    // --- PART 1: Types that implement 'Copy' ---
    // Booleans are stored entirely on the stack.
    let registrations = (true, false, true);

    // 'first' becomes the owner of a COPY of the data.
    // The original tuple 'registrations' remains fully intact and valid.
    let first = registrations.0;
    println!("{first} and {registrations:?}");

    // --- PART 2: Types that do NOT implement 'Copy' ---
    // Strings store their data on the heap.
    // The variable 'languages' owns the pointers to that heap memory.
    let languages = (String::from("Rust"), String::from("JavaScript"));

    // By using '&', we are BORROWING the value.
    // 'first' is now a reference (&String), not the owner.
    let first = &languages.0;

    // Because we borrowed instead of moved, 'languages' is still the owner
    // and can be printed in its entirety here.
    println!("{first} and {languages:?}");

    // NOTE: If we had written 'let first = languages.0;',
    // ownership would MOVE to 'first', and 'languages' would become
    // partially uninitialized and unusable.
}
