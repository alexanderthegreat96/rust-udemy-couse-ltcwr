#[allow(unused_variables)]
fn main() {
    // an unit is an empty tuple
    let result = mystery();

    // it simply returns nothing
    // it's the default return value
    // of a function if no return value
    // is specified
    let second_result = another_mistery();
}

fn mystery() {
    println!("Hello there");
}

// you can even annotate it
// by doing -> ()
fn another_mistery() -> () {
    println!("Nothing")
}
