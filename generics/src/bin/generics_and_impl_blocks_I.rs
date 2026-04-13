#![allow(dead_code)]
#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

struct Metadata {
    amount: f32,
    has_gold: bool,
    has_silver: bool,
}

// we gotta provide the type for the implementation
// since treasure chest accepts a type T
// then each implementation will be valid for that specific type
// meaning that we can have different implementations with the same name
// and different methods for different types
// warning you cannt use the same method names
// unless you define a trait

// this is for a String
impl TreasureChest<String> {
    fn clean_treasure(&mut self) {
        self.treasure = self.treasure.trim().to_string();
    }
}

// implementation for a string literal array with 3 string slices
impl TreasureChest<[&str; 3]> {
    fn amount_of_treasure(&self) -> usize {
        self.treasure.len()
    }
}

// we're using a custom struct here to define the generic behavior
impl TreasureChest<Metadata> {
    fn print_data(&self) {
        println!("Treasure Metadata");
        println!(
            "{} | {} | {}",
            self.treasure.has_gold, self.treasure.has_silver, self.treasure.amount
        );
    }
}

fn main() {
    let gold_chest = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: "Gold",
    };
    println!("{:?}", gold_chest);

    let mut silver_chest = TreasureChest {
        captain: String::from("Bloodsail"),
        treasure: String::from("     Silver    "),
    };
    silver_chest.clean_treasure();
    println!("{:?}", silver_chest);

    let special_chest = TreasureChest {
        captain: String::from("Bootyplunder"),
        treasure: ["Gold", "Silver", "Platinum"],
    };
    println!("{:?}", special_chest.amount_of_treasure());
    println!("{:?}", special_chest);

    // using the custom struct
    let my: TreasureChest<Metadata> = TreasureChest {
        captain: String::from("Barbarosa"),
        treasure: Metadata {
            amount: 11321.2,
            has_gold: true,
            has_silver: false,
        },
    };

    my.print_data();
}
