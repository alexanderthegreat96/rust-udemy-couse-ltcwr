fn main() {
    // 'first_name' is a string slice (&str) with a 'static lifetime.
    // Even though it's defined inside a block, the underlying data lives
    // in the "Data Segment" of the compiled binary, not on the stack.
    let first_name: &str = {
        // 'action_hero' points to a hardcoded string literal.
        // String literals have the type &'static str.
        let action_hero: &str = "Arnold Schwarzenegger";

        // We return a slice of the literal. Because the source is 'static,
        // the slice is also 'static. This is why it can outlive this block.
        &action_hero[0..6]
    };

    // Prints "Arnold" — the data is still valid because the binary
    // is still loaded in memory.
    println!("{first_name}");
}

