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
    // the result enum has some methods
    // is_err or is_ok
    // we can use these to verify the status of the execution
    let result = divide(10.0, 5.0);
    println!("{}", result.is_ok());
    println!("{}", result.is_err());

    // we can assign usin let Ok(())
    if let Ok(()) = connect_to_db() {
        println!("We are connected!");
    }

    // use is_ok
    let connect_db = connect_to_db();
    if connect_db.is_ok() {
        println!("We're connected to the database!");
    }

    let connect_db = connect_to_db();
    if connect_db.is_err() {
        println!("We got an error connecting to the database!");
    }
    // just crashes the program
    // // gut we got the error
    let connect_db = connect_to_db().expect("Unable to connect to the database");
    println!("{:?}", connect_db);
}
