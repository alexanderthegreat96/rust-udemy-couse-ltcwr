fn main() {
    // methods that mutate the vector
    //
    let mut pizza_diameters = vec![8, 10, 12, 14];
    // push adds a new element after the last previous element
    pizza_diameters.push(16);
    pizza_diameters.push(18);

    // adds a value at a given index
    pizza_diameters.insert(0, 4);

    // delete the last element
    let last_pizza_diameter = pizza_diameters.pop();
    println!("{last_pizza_diameter:?}");
    
    // remove removes it from a given index
    let third_diameter_from_start = pizza_diameters.remove(2);
    println!("{third_diameter_from_start}"); // 10
    println!("{pizza_diameters:?}");

    // this one panics because that index does not exist
    pizza_diameters.remove(50);
}
