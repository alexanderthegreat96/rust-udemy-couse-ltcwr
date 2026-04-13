fn main() {
    // more practical use of unwrap

    let nums: Vec<i32> = Vec::from([1, 2, 3, 4]);

    let get_four = get_number(nums, 4);
    if get_four.is_some() {
        println!("We found four");
    }

    // we found the value so we can safely unwrap
    // as we check on top
    let found_four = get_four.unwrap();
    println!("This is our value: {}", found_four);

    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    // calling unwrap on a non-null value
    // will cause a runtime error
    // so we want to prevent that
    // this way, we can simply return
    let bass: Option<&String> = musical_instruments.get(2);
    println!("{:?}", bass);
    let valid_instrument = bass.expect("Unable to retrieve element");
    println!("{valid_instrument}");

    let invalid_instrument = musical_instruments.get(100);
    println!("{:?}", invalid_instrument);
    invalid_instrument.expect("Unable to retrieve musical instrument");
}

fn get_number(numbers: Vec<i32>, num: i32) -> Option<i32> {
    for n in numbers {
        if n == num {
            return Some(num);
        }
    }

    None
}
