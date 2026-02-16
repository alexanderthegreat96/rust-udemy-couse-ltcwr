/// Represents milk types with different data structures.
#[derive(Debug)]
enum Milk {
    Whole,
    Lowfat(i32),
    NonDairy { kind: String },
}

fn main() {
    // We initialize a NonDairy variant.
    let my_beverage = Milk::NonDairy {
        kind: String::from("Oat"),
    };

    // The 'let-else' construct:
    // 1. It attempts to match 'my_beverage' against the NonDairy pattern.
    // 2. SUCCESS: It binds 'kind' to the local scope (unlike 'if let', which limits scope to the block).
    // 3. FAILURE: It executes the 'else' block, which MUST diverge (return, break, or panic).

    // basically, the else block gets executed if the variable
    // my_beverage is not equal to NonDiary
    let Milk::NonDairy { kind } = my_beverage else {
        // This block only runs if my_beverage is NOT NonDairy.
        println!("You do not have the nondairy milk");
        return; // Early exit is required here.
    };

    // Because we used 'let-else', 'kind' is now available in the main scope!
    println!("{kind} milk is available here");

    let my_beverage = Milk::Lowfat(5);

    let Milk::Lowfat(percent) = my_beverage else {
        println!("You don't have LowFat milk");
        return;
    };

    println!("Your low fat milk has {percent}% only");
}
