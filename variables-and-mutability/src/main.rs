// variables and mutability

// dereference to update the value at the memory address
fn change_value(number: &mut i32) {
    // standard dereferencing
    *number = 60;
}

// explicit lifetimes needed to link the item's validity to the vector's storage
fn add_fruit<'a>(item: &'a str, data: &mut Vec<&'a str>) {
    data.push(item);
}

fn main() {
    let name: &str = "Alex";
    let mut fruits = vec!["apples", "oranges"];
    let mut number: i32 = 16;

    println!("Some string: {}", name);
    println!("Number before change: {:?}", number);

    change_value(&mut number);

    println!("Number after change: {:?}", number);

    print!("Fruits before adding more:\n");
    // borrow the vector to iterate without dropping it (moving ownership)
    for fruit in &fruits {
        println!("{:?}", fruit);
    }

    add_fruit("pineapples", &mut fruits);
    add_fruit("strawberries", &mut fruits);
    add_fruit("kiwis", &mut fruits);
    add_fruit("dragonfruits", &mut fruits);
    add_fruit("mangos", &mut fruits);

    print!("Fruits after adding more:\n");

    // shared borrow is enough here since we aren't mutating inside the loop
    for fruit in &fruits {
        println!("{:?}", fruit);
    }

    // some unsured vars
    let people: i32 = 60;

    // println supports providing indexes for the value placeholders
    // instead of {} you can do: {0}, {1} etc
    println!(
        "Hi, I am {0}. I used to have {1} fruits. Now I only have: {2}. Why the fuck do I only have {2}?",
        name,
        number,
        fruits.len()
    );

    // variables can be ignored by using _
    // in this case i also moved ownership
    // so people cannot really be used anymore
    let _ = people;

    // another example would be this
    // start the variable with an underscore
    let _citizens: i32 = 100;
}
