use rand::{rngs::ThreadRng, seq::IndexedRandom};

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

fn connect_to_db() -> Result<(), &'static str> {
    let mut rng: ThreadRng = rand::rng();
    let choices: [bool; 2] = [true, false];

    // normally returns
    // Option<&bool>
    let status: &bool = choices.choose(&mut rng).unwrap();

    if !*status {
        return Err("Server went away!");
    }

    Ok(())
}

fn main() {
    let result = divide(10.0, 2.0);

    match result {
        Ok(calculation) => println!("Result: {}", calculation),
        Err(message) => println!("Error: {}", message),
    }


    let connect: Result<(), &str> = connect_to_db();
    match connect {
        Ok(()) => {
            println!("Succesfuly connected to database!");
        },
        Err(msg) => {
            println!("Cannot connect to database: {}", msg);
        }
    }

}
