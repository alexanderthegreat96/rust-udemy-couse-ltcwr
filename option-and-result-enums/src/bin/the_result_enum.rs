use rand::{rngs::ThreadRng, seq::IndexedRandom};

fn main() {
    // models the outcome
    // that can either reurn success or error
    // Result<T, Err>
    // 2 variants: Ok
    // Err
    //
    // other programming languages use exceptions
    // rust does not, but Result is the same thing
    let ok: Result<i8, &str> = Ok(5);
    println!("{:?}", ok);
    let disaster: Result<i32, &str> = Err("Something went wrong");
    println!("{:?}", disaster);

    let db_con: Result<(), &'static str> = connect_to_db();
    println!("{db_con:?}");
    if db_con.is_err() {
        println!("Unable to connect!");
    } else {
        println!("Connected to database!");
    }
}

// Result<T, E> -> in this case T is a tuple
// since do not return anything
// and then E is a static string
fn connect_to_db() -> Result<(), &'static str> {
    let mut rng: ThreadRng = rand::rng();
    let choices: [bool; 2] = [true, false];

    // normally returns
    // Option<&bool>
    let status: &bool = choices.choose(&mut rng).unwrap();

    if !*status {
        return Err("Unable to connect");
    }

    Ok(())
}
