fn main() {
    // 1. Stack Allocation
    // This integer is stored directly on the stack.
    let my_stack_value = 2;

    // 2. Immutable Borrowing
    // We create a reference (&) to the stack value.
    // This doesn't take ownership; it just "points" to the data.
    let my_integer_reference = &my_stack_value;

    // 3. Manual Dereferencing
    // Using '*' follows the pointer to get the actual value (2).
    println!("Dereferenced value: {}", *my_integer_reference);

    // Note: Rust often handles dereferencing automatically in println!,
    // but using * explicitly shows you're accessing the underlying data.
    println!("Value via reference: {}", my_integer_reference);

    // 4. Heap Allocation & Referencing
    // Strings are stored on the heap; the variable 'my_heap_value' lives
    // on the stack and holds a pointer to that heap memory.
    let my_heap_value = String::from("Toyota");
    let my_heap_reference = &my_heap_value;
    println!("Heap reference: {}", my_heap_reference);

    // --- Dereferencing & Mutation ---

    // 5. Mutable References
    // To change a value through a reference, the variable must be 'mut'
    // and we must create a mutable reference (&mut).
    let mut my_value: i32 = 189;
    println!("Original value: {}", my_value);

    // We pass a mutable reference to the function.
    mutate_value(89, &mut my_value);
    println!("Mutated value: {}", my_value);
}

/// Changes the value at the memory address 'mutable' to 'value'.
fn mutate_value(value: i32, mutable: &mut i32) {
    // The '*' operator here is crucial. We aren't changing the pointer;
    // we are changing the data sitting at that address.
    *mutable = value;
}
