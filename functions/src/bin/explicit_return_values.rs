fn main() {
    // explicit return values
    // return values is what the function returns back to the user
    // so we simply annotate the function return value with it's return type
    // for example, a function of sum takes 2 intergers and returns another integer
    let result = square(5);
    println!("The square of 5 is {result}");

    let result = square(13);
    println!("The square of 13 is {result}");

    let result: i32 = sum(5, 10);
    println!("The sum of 5 + 10 is: {}", result);
}

fn square(number: i32) -> i32 {
    return number * number;
}

// takes 2 integers
// returns another integer
fn sum(first: i32, second: i32) -> i32 {
    return first + second;
}
