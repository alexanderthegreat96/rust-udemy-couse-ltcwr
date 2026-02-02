fn main() {
    let oranges = String::from("Oranges");
    print_my_value(&oranges); // let value = oranges;
    println!("{oranges} is now valid");

    // now. the same behavior works differently with types which implement the copy trait
    // therefore, with i32, it will work
    // this is because here a copy occurs and not a move
    let apples: i32 = 20;
    print_another_value(apples);
    println!("Apples is still valid");
}

// because i didnt specify this is a reference
// my function will actually take ownership
// so anything called after this function is basically invalid
// to fix, we have to add & before the type
// so instead of String, we have &String
// we could also do val.clone() which provides a copy
// string does not implement copy, so a move
// occurs when not using a refferences
// therfore, the function gets ownership
// and when the functiojn block ends
// the value goes out of scope
fn print_my_value(value: &String) {
    println!("Your value is {value}");
}

// i32 implements copy
fn print_another_value(value: i32) {
    println!("The values is: {value}")
}
