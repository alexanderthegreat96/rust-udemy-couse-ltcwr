use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// a constant is similar to a variable
// it's always immutable, hence the name constant
const TAX_RATE: f64 = 7.25;
// in both cases 'static lifetimes are used
// the first case from line 8 is actually implicit, while in line 9, it is implicit
const GAMES: [&str; 3] = ["cyberpunk", "watch_dogs", "gta_v"]; // recomended way for a know slice of strings
const CITIES: [&'static str; 3] = ["warsaw", "montreal", "new york"]; // explicit way

fn generate_random_str(mut max_len: i32) -> String {
    let mut nanos: u128 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let mut growable_string = String::new();

    const CHAR_LIST: [&str; 26] = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];

    if max_len > 15 {
        max_len = 6;
    }

    for _ in 0..max_len {
        nanos = nanos.wrapping_mul(6364136223846793005).wrapping_add(1);
        let index = (nanos as usize) % 26;
        growable_string.push_str(CHAR_LIST[index]);
    }

    return growable_string;
}

fn generate_random_map(max_len: i32, max_str_len: i32) -> HashMap<String, String> {
    let mut hash_map: HashMap<String, String> = HashMap::new();
    for _ in 0..max_len {
        let rand_k: String = generate_random_str(max_str_len);
        let rand_v: String = generate_random_str(max_str_len);
        hash_map.insert(rand_k, rand_v);
    }

    return hash_map;
}

fn main() {
    let income: i32 = 100000;

    // create a hashmap / dictionary
    // where: key is a string and value is a string as well
    let mut games_production: HashMap<&str, &str> = HashMap::new();
    for i in 0..3 {
        games_production.insert(GAMES[i], CITIES[i]);
    }

    println!("Games produced: {:?}", games_production);
    println!("My income is {income} and my tax rate is {TAX_RATE}");

    // constants can be declared within functions
    // rust convention for naming: ALL_CAPITAL
    // the required EXPLICIT TYPE DECLARATION
    // ex: CONST_NAME: TYPENAME = "something"
    // for vars, rust ussually infers the data type
    // but for constants, it's required to have a type annotation

    const MAX_AGE: i32 = 80;
    println!("The max age is: {MAX_AGE}");

    let random_map: HashMap<String, String> = generate_random_map(100, 10);

    println!("I just generated a map: {:?}", random_map);
}
