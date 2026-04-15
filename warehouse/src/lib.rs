pub mod inventory;
pub mod orders;

pub use inventory::{ProductCategory, Item};
pub use inventory::{MANAGER, FLOOR_SPACE};
pub use inventory::MANAGER as INVENTORY_MANAGER;
pub use orders::MANAGER as ORDER_MANAGER; // use "as" and rename the const basica
