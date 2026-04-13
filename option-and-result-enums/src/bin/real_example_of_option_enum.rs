use lazy_static::lazy_static;
use std::collections::HashMap;

fn main() {
    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    // basically returns an optional refference to string
    let bass: Option<&String> = musical_instruments.get(2);
    println!("{:?}", bass);

    // not found, or is none
    let invalid_instrument = musical_instruments.get(100);
    println!("{:?}", invalid_instrument);

    // testing getting a student by id

    let student: Option<&'static String> = get_student(2);

    if student.is_some() {
        println!("We found a student");
    }

    let another_student: Option<&'static String> = get_student(10);
    if another_student.is_none() {
        println!("We could not find a student!");
    }
}

fn get_student<'a>(student_id: u32) -> Option<&'a String> {
    // rust does not support hashmaps as heap allocations
    // so we are using lazystatic lib for this
    lazy_static! {
        static ref STUDENT_LIST: HashMap<u32, String> = {
            let mut map = HashMap::new();
            map.insert(1, String::from("Mike"));
            map.insert(2, String::from("James"));
            map.insert(3, String::from("Dan"));
            map.insert(4, String::from("Alex"));
            map
        };
    }

    let student: Option<&'a String> = STUDENT_LIST.get(&student_id);
    student
}
