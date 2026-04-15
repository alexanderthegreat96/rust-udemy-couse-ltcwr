/*
Define a `make_money` function that accepts a mutable
String reference. The function should concatenate
the characters "$$$" to the end of the String.
Invoke the function in `main`.

Define a `trim_and_capitalize` function that accepts
a string slice. It should return a String with
all whitespace removed and all characters in uppercase.
Invoke the function in `main`.

Define an `elements` function that accepts a string
slice. It should split the string by all occurrences
of the `!` symbol and return a vector of the string
slices. Invoke the function in `main`.

Example:
elements("Gold!Silver!Platinum")
=> Vector of ["Gold", "Silver", "Platinum"]

Define a `get_identity` function. The function should
ask the user for their first and last name in TWO
steps (i.e., collect user input twice). Make sure
to communicate the instructions to the user.
For each Result enum you receive, call the `expect`
method and provide a custom error message (like
"Failed to collect first name"). Return a String
with the first and last names combined. Invoke
the `get_identity` function in `main`, and output the
returned String.

Example:
fn main() {
  let name = get_identity();
   println!("{name}"); // Bill Murray
}
*/
use std::io;

fn make_money(input: &mut String) {
    let append_string: String = " $$$".to_string();
    input.push_str(&append_string); 
}

fn trim_and_capitalize(input: &str) -> String {
    let trimmed = input.trim();
    let trimmed = trimmed.trim_start();
    let trimmed = trimmed.trim_end();
    return trimmed.to_uppercase();
}

fn split_by_exclamation_point(input: &str) -> Vec<&str> {
    let split = input.split('!');
    let collect = split.collect();
    return collect;
}

fn get_identity(name_buffer: &mut String) {
    println!("Just type your name: ");
    let input = io::stdin().read_line(name_buffer) ;
    if input.is_err() {
        println!("Unable to grab user input");
    }
}

fn main() {
    let mut money = String::from("112.12");
    make_money(&mut money);
    println!("{}", money);

    let some_random_str_slice: &str = "Tokio   , New York, Nevada";
    println!("Cleaned string slice: {}", trim_and_capitalize(some_random_str_slice));
    println!("Split this: {:?}", split_by_exclamation_point("Gold!Silver!Platinum"));

    let mut input_buffer = String::new();
    get_identity(&mut input_buffer);

    println!("Your name is: {}", input_buffer);
}
