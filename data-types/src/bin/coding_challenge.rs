/*
Declare an i32 variable assigned to 1337.
Use the underscore character to add a visual
separator between the numbers.

Cast the i32 to an i16 integer and assign the result
to a separate variable.

Declare a floating-point value of your choosing.
Print out the number with 3 digits of precision.

Declare a 'with_milk' variable set to a Boolean.
Declare a 'with_sugar` variable set to a Boolean.

Declare a 'is_my_type_of_coffee` variable. It should
be set to true if the coffee has both milk and sugar.

Declare an `is_acceptable_coffee` variable. It should
be set to true if the coffee has either milk or
sugar.

Declare an array with four i8 integers of your choosing
Print out the array in its Debug representation.

Declare a tuple consisting of the integer, float,
a Boolean, and the array that you previously declared.
Print out the tuple in its Debug representation.
*/

fn main() -> () {
    let my_value: i32 = 1337;
    let my_second_value = my_value as i16;

    println!("My values: {}, {}", my_value, my_second_value);

    let exchange_rate: f64 = 2.33233;
    println!("Exchange rate is: {:.2}", exchange_rate);

    let with_milk: bool = true;
    let with_sugar: bool = true;

    let is_my_type_of_coffee: bool = with_milk && with_sugar;
    println!("My type of coffe: {}", is_my_type_of_coffee);

    let is_acceptable_coffee: bool = with_milk || with_sugar;
    println!("Acceptable coffee: {}", is_acceptable_coffee);

    let four_ints: [i8; 4] = [1, 2, 3, 4];
    dbg!(four_ints);

    let man: (i32, f64, bool) = (30, 6.3, true);
    dbg!("{}", man);
}
