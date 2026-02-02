/*
Declare a `is_concert` variable set to a boolean.
Declare a `is_event` variable assigned to `is_concert`.
Will Rust move ownership? State your answer, then confirm
by trying to printing both variables out.

Declare a `sushi` variable to set to a string literal of "Salmon"
Declare a `dinner` variable assigned to the `sushi` variable.
Will Rust move ownership? State your answer, then confirm
by trying to printing both variables out.

Repeat the previous example but use a heap String instead.
Will Rust move ownership? Explain why the result is different
from the previous operation.

The `clear` method modifies a heap String to have no content.
Declare an `eat_meal` function that accepts a `meal` parameter
of type String. In the body of `eat_meal`, invoke the `clear`
method on the `meal` parameter.

In the `main` function, invoke the `eat_meal` function and pass
in your "Salmon" String. Explain what happens when the eat_meal
function runs. Describe the complete movement of ownership of
the "Salmon" String throughout the program.

Say we want to keep the String around after `eat_meal` is
called. How can we continue to have access to the String in
the `main` function? Print out the (empty) String.
*/

fn main() {
    // no it won't
    // it will simply copy
    let is_concert: bool = true;
    let is_event: bool = is_concert;

    // nope, simply copies it
    let sushi: &str = "Salmon";
    let dinner = sushi;

    // it moves it here
    // because string does not implement the copy Trait
    let fish: String = String::from("salmon");
    let incredient = fish;

    // meal is the owner of pizza
    let mut meal: String = String::from("pizza");
    println!("Now eating: {}", meal);

    // now meal is the owner again
    meal = eat_meal(meal);
    // to keep the string you do what i did above
    // you return ownership
    // this is bad practice
    // if you want to mutate, simply use a mutable reference
    println!("Pizza should be gone: {}", meal);
}

// ownership mvoes to the meal function parameter
// which gives it back by returning it
fn eat_meal(mut meal: String) -> String {
    meal.clear();
    meal
}
