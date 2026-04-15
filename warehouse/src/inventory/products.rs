use fake::Dummy;

#[derive(Debug, Dummy)]
pub enum ProductCategory {
    Ladder,
    Hammer
}

// in order to handle structs externally
// we gotta declare the struct and the fields public
// the situation where you would only declare item as public but 
// the fields private
// is if we have an Impl. and the impl manages the fields
// while you from the outside only call the public methods
#[derive(Debug, Dummy)]
pub struct Item {
    pub name: String,
    pub category: ProductCategory,
    pub quantity: u32
}

impl Item {
    pub fn new(name: String, category: ProductCategory, quantity: u32) -> Self {
        Self {
            name,
            category,
            quantity
        }
    }
    
    // super:: references the parent module
}
