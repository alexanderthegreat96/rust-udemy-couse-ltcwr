
use warehouse::{FLOOR_SPACE, INVENTORY_MANAGER, ORDER_MANAGER};

fn main() {
    println!("Our managers are {} and {}, and we have {} square feet of space.", INVENTORY_MANAGER, ORDER_MANAGER, FLOOR_SPACE);
}
