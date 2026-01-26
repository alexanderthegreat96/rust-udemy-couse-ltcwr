/*
Define a `color_to_number` function that accepts a 'color'
parameter (a string). Use if, else if, and else
statements to return a corresponding numeric value based
on the following rules:
1. If the color is "red", return 1.
2. If the color is "green", return 2.
3. If the color is "blue", return 3.
4. If the color is any other string, return 0.

Refactor the function above to use the `match` statements
instead of if, else if, and else.

Define a `factorial` function that calculates the
factorial of a number. The factorial is the product
of multiplying a number by every incremental
number leading up to it, starting from 1.

Examples:
The factorial of 5 is 5 * 4 * 3 * 2 * 1 = 120
factorial(5) should return 120.

The factorial of 4 is 4 * 3 * 2 * 1 = 24
factorial(4) should return 24.

Implement two solutions/functions for the problem.
The first solution should not use recursion.
The second solution should use recursion.
*/

fn color_to_number(color: &str) -> i32 {
    let lower_color: String = color.to_lowercase();
    let binding: &str = lower_color.as_str();
    if binding == "red" {
        return 1;
    } else if binding == "green" {
        return 2;
    } else if binding == "blue" {
        return 3;
    } else {
        return 0;
    }
}

fn match_color(color: &str) -> i32 {
    let lower_color: String = color.to_lowercase();
    let binding: &str = lower_color.as_str();
    match binding {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

fn factorial(number: i32) -> i32 {
    if number <= 1 {
        return 1;
    }

    return number * factorial(number - 1);
}

fn main() {
    let colors: [&str; 4] = ["red", "green", "blue", "yellow"];
    for color in colors {
        println!("Color: {} is: {}", color, color_to_number(color));
    }

    println!("------------");

    for color in colors {
        println!("Color: {} is: {}", color, match_color(color));
    }

    println!("Factorial: {}", factorial(5));
}
