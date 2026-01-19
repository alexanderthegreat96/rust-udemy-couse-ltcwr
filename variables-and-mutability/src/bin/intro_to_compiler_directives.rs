// a compiler directive is
// an instruction we give the compiler
// on how to parse the code

// the macro below will suppress warnings
// related to leaving unused variables in the code
#![allow(unused_variables)] // this applies for the entire file
type Meters = i32;

// this applies the directive on the function below
// so we use them above anything we want to showcase
// or suppress
#[allow(dead_code)]
fn test_func() {}

// you can put a directive above everything
// and it will apply to the entire file

fn main() {
    let mile_race_length: Meters = 1600;
    let two_mile_race_length: Meters = 3200;
    let names: [&str; 3] = ["mike", "ana", "dan"];

    // #[allow(unused_variables)]
    // this applies within functions
}
