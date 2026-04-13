#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

impl TreasureChest<String> {
    fn clean_treasure(&mut self) {
        self.treasure = self.treasure.trim().to_string();
    }
}

impl TreasureChest<[&str; 3]> {
    fn amount_of_treasure(&self) -> usize {
        self.treasure.len()
    }
}

// normally, simply declaring impl TreasureChest<T> will not work
// because rust is looking for a concrete type
// if, however, there is a trully generic type requirement
// say a function that has to be used across all types
// then we use impl<T> TreasureChest<T>

impl<T> TreasureChest<T> {
    fn capital_captain(&self) -> String {
        self.captain.to_uppercase()
    }
}

struct Number<T> {
    number: T,
}

impl<T> Number<T>
where
    T: std::fmt::LowerHex,
{
    fn as_hex(&self) -> String {
        format!("{:x}", self.number)
    }
}

impl<T> std::fmt::LowerHex for Number<T>
where
    T: std::fmt::LowerHex,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.number, f)
    }
}

fn main() {
    let gold_chest = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: "Gold",
    };
    println!("{}", gold_chest.capital_captain());

    let mut silver_chest = TreasureChest {
        captain: String::from("Bloodsail"),
        treasure: String::from("     Silver    "),
    };
    silver_chest.clean_treasure();
    println!("{}", silver_chest.capital_captain());

    let special_chest = TreasureChest {
        captain: String::from("Bootyplunder"),
        treasure: ["Gold", "Silver", "Platinum"],
    };
    println!("{:?}", special_chest.amount_of_treasure());
    println!("{:?}", special_chest.capital_captain());
    println!("{:?}", special_chest);

    let my_int: Number<i32> = Number { number: 120 };
    println!("Int as hex: {}", my_int.as_hex());
}
