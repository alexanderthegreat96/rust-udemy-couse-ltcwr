/// Represents different types of milk using an Enum.
/// This demonstrates the three types of variants: Tuple, Unit, and Struct-like.
enum Milk {
    LowFat(i32),               // Tuple variant: stores an integer (percentage)
    Whole,                     // Unit variant: stores no data
    NonDairy { kind: String }, // Struct-like variant: stores a named field
}

fn main() {
    // 1. Working with a Unit variant (Whole)
    // We initialize the variable with a specific variant
    let my_beverage = Milk::Whole;

    // 'if let' checks if 'my_beverage' matches the 'Milk::Whole' pattern.
    // It's a concise alternative to a 'match' statement when you only care about one case.
    if let Milk::Whole = my_beverage {
        println!("You have whole milk");
    }

    // 2. Working with a Tuple variant (LowFat)
    let my_beverage = Milk::LowFat(4);

    // Here, 'if let' destructures the variant and binds the inner value
    // to the variable 'percent' for use inside the block.
    if let Milk::LowFat(percent) = my_beverage {
        println!("You chose a low fat with percentage: {}", percent);
    }

    // 3. Working with a Struct-like variant (NonDairy)
    let my_beverage = Milk::NonDairy {
        kind: String::from("Oat Milk"),
    };

    // This destructures the named field 'kind'.
    // Note: The variable name inside the {} must match the field name in the enum definition.
    if let Milk::NonDairy { kind } = my_beverage {
        println!("You chose: {}", kind);
    }
}
