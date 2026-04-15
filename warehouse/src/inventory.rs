
// we can also make use of submodules
// first option is declaring the mod keyword and moving content around
pub mod products;
// using pub use allows us to use these from a top level domain
// without having to reach within the parent module first
pub use products::{Item, ProductCategory};

// in order to use anything from within
// we gotta use the inventory namespace
// using :: <- the scope resolution operator
// also, we gotta declare things public, if we want to use them
// outside of a module
// module as a file basically
// inventory is used in main.rs

pub const FLOOR_SPACE: i32 = 10000;
pub const MANAGER: &str = "Ivan Inventory";

pub fn talk_to_manager() {
    println!("Yo, {MANAGER}, how is your coffee? What do you think of {:?}?", ProductCategory::Ladder);
}

// obviously, this needs to be made public
// since it's a submodule
// and you won't be able to access them



