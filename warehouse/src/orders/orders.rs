// in main.rs 
// we declared: mod orders;
// since rust resolves the module name by file name
// it will now be exposed to main.rs
pub const MANAGER: &str = "Oliver Orderson";

pub fn talk_to_manager() {
    println!("Yo, {MANAGER}, how is your coffee?");
}

