use std::time::{SystemTime, UNIX_EPOCH};

struct Empty; // this has no data inside
struct Helpers; // so we got a helpers struct that can contain helper methods

impl Helpers {
    // vector of type T which is an iterator
    // is it empty?
    fn is_empty<T>(vec: Vec<T>) -> bool {
        return vec.len() < 1;
    }

    fn generate_uuid() -> String {
        let start: u128 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos();
        format!("{:x}", start)
    }
}

fn main() {
    // unit like structs
    // a unit is a tuple that holds no values
    // -> ()
    #[allow(unused_variables)]
    let nothing: () = (); // classic unit

    // the reason why they exist is to define methods
    // on a type that has no fields
    // so you can structure functions there
    #[allow(unused_variables)]
    let my_empty_sruct = Empty; // nothing here, same as above

    let empty_vector: Vec<i32> = Vec::new();

    if Helpers::is_empty(empty_vector) {
        println!("This is an empty list.");
    }

    println!("Generated ID: {}", Helpers::generate_uuid());
}
