fn main() {
    // a refference allows the program to use a value
    // without moving ownership
    // this is also called borrowing
    // technically, it's the address where the data lives
    // &mut something is a mutable refference
    // which we can use to modify the data if needed
    let my_stack_value = 2;

    // this is the owner of the refference
    // not the value itself

    // a reference and a pointer are used interchangeably
    // that is becaue a reference is a type of pointer
    // it points to the place in memory where that is
    // in rust, there is a guarantee that the reference
    // points to what it has to
    let my_integer_reference = &my_stack_value;

    println!("{}", my_integer_reference);

    let mut my_heap_value = String::from("Toyota");
    println!("Heap value before: {my_heap_value}");
    change_string(&mut my_heap_value);
    println!("Heap value after: {}", my_heap_value);

    // very important!
    // references must never outlive their referent
}

pub fn change_string(owned: &mut String) {
    owned.push_str(" is great!");
}
