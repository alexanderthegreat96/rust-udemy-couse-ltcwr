/// Represents types of milk with associated data or unit variants.
enum Milk {
    LowFat(i32), // Tuple variant carrying a percentage value
    WholeMilk,   // Unit variant
}

impl Milk {
    /// Consumes the Milk instance and performs pattern matching.
    /// By taking 'self', we move ownership into this method.
    fn drink(self) {
        match self {
            // SPECIFIC MATCH: Rust matches from top to bottom.
            // We must put specific values (like 2) before the general variable binding.
            Milk::LowFat(2) => {
                println!("Delicious 2% milk is my favorite!");
            }

            // GENERAL MATCH: This 'percent' variable binds to any i32 value
            // that wasn't caught by the specific matches above.
            Milk::LowFat(percent) => {
                println!("You've got the low fat: {percent}%.");
            }

            // UNIT MATCH: Handles the WholeMilk variant.
            Milk::WholeMilk => {
                println!("You've got the whole milk.");
            } // NOTE: No '_' (catch-all) is needed here because all
              // possible variants of the Milk enum are explicitly handled.
              // This makes the match "exhaustive."
        }
    }
}

fn main() {
    // Calling the method on a specific instance.
    // Because 'drink' takes ownership, these instances are dropped after the call.
    Milk::LowFat(2).drink();
    Milk::WholeMilk.drink();
}
