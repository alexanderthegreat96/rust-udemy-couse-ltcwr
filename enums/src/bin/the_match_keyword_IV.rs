// The derive attribute tells the compiler to automatically generate
// code for the Debug trait, allowing you to print the enum using {:?}
#[derive(Debug)]
enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl OnlineOrderStatus {
    // Note the '&self' - this takes a reference rather than ownership.
    // This allows you to call .check() without losing the ability
    // to use the enum variable later in the program.
    fn check(&self) {
        match self {
            // Specific case: Item completed the journey
            OnlineOrderStatus::Delivered => {
                println!("Your item has arrived");
            }
            // Specific case: Initial status
            OnlineOrderStatus::Ordered => {
                println!("You have just ordered the item");
            }
            // The catch-all pattern (Underscore):
            // This handles Shipped and Packed in one block.
            // In Rust, this makes the match 'exhaustive', satisfying the compiler.
            _ => {
                println!("Your item is not there yet");
            }
        }
    }
}

fn main() {
    // We create a 'Shipped' variant and call check.
    // It will fall through to the '_' match arm.
    OnlineOrderStatus::Shipped.check();

    // This will match the 'Ordered' arm exactly.
    OnlineOrderStatus::Ordered.check();
}
