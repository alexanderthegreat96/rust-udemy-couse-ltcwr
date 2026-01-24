/*
Define an apply_to_jobs function that accepts a
'number' parameter (an i32) and a 'title' parameter
(a string). It should print out the string:
"I'm applying to {number} {title} jobs".

Example:
apply_to_jobs(35, "Rust Developer")
-> "I'm applying to 35 Rust Developer jobs"

Define an is_even function that accepts a 'number'
parameter (an i32). The function should return a true
if the number is even and a false if the number is
odd.
Examples:
is_even(8) -> true
is_even(9) -> false

Define an alphabets function that accepts a 'text'
parameter (an &str). The function should return a
tuple of two Booleans. The first Boolean should check
if the text contains the letter 'a'. The second
Boolean should check if the text contains the letter
'z'. You can use the 'contains' method to check if a
string contains a specific character. See the documentation:
https://doc.rust-lang.org/std/primitive.str.html#method.contains

Examples:
println!("{:?}", alphabets("aardvark")); -> (true, false)
println!("{:?}", alphabets("zoology"));  -> (false, true)
println!("{:?}", alphabets("zebra"));    -> (true, true)
*/

use std::ops::RangeInclusive;

fn is_even(number: i32) -> bool {
    // if the remainder of the division is 0
    // the it's even
    return number % 2 == 0;
}

fn apply_to_jobs(number: i32, title: String) -> () {
    println!("I'm applying to {number} {title} jobs");
}

fn alphabets(text: String) -> (bool, bool) {
    return (text.contains('a'), text.contains('z'));
}

// main here also returns an unit
// which si almost the same as void in other languages
fn main() -> () {
    apply_to_jobs(100, String::from("engineering"));

    for num in 0..=10 {
        if is_even(num) {
            println!("Number: {num} is even");
        } else {
            println!("Number: {num} is not even");
        }
    }

    // using RangeInclusive here
    // because we are also including z
    let alphabet: RangeInclusive<char> = 'a'..='z';

    let result = alphabets(alphabet.into_iter().collect());
    println!("{:?}", result);

    println!("{:?}", alphabets(String::from("aardvark")));
    println!("{:?}", alphabets(String::from("zoology")));
    println!("{:?}", alphabets(String::from("zebra")));
}
