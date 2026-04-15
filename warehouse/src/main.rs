#![allow(unused)]
// 1. Module Declarations 
// (Tells Rust to look for inventory.rs and orders.rs)
// mod inventory;
// mod orders;

use fake::{Fake, Faker};

// 2. Scoping Shortcuts
// (Brings items into the local scope so we don't have to type full paths)
// use inventory::{ProductCategory, Item};
// use inventory::{MANAGER, FLOOR_SPACE};
// use orders::MANAGER as ORDER_MANAGER; // use "as" and rename the const basically

use warehouse::*;

fn main() {
    // --- Accessing Module Constants & Functions ---
    println!("The manager of our inventory is {}", MANAGER);
    inventory::talk_to_manager();
    
    println!("The manager of our orders is {}", ORDER_MANAGER);
    orders::talk_to_manager();

    println!("We have {} square feet of floor space.", FLOOR_SPACE);

    // --- Using Imported Types ---
    let category = ProductCategory::Hammer;
    let tall_ladder = Item::new( String::from("Tall Ladder"), ProductCategory::Ladder, 100);
    println!("My product category: {:?}", category);
    println!("The tall ladder: {:?}", tall_ladder);

    // --- Path Examples ---
    // Absolute path (starts from the crate root)
    println!("Manager name via crate root: {}", MANAGER);

    let fake_item: Item = Faker.fake();
    println!("{:?}", fake_item);

    let random_category: ProductCategory = Faker.fake();
    println!("{:?}", random_category);

    // the standard library
    // the glob operator *
    // used using the use statement to import everything public from a module
    // use std::collections::*
    // rust can have 1 binary crate
    // rust can have 1 binary and 1 library crate
    // rust can have multiple binary crate
    // rust can have 1 library crate and multiple binary ones
    // for multiple binaries -> create a bin folder
}
