fn main() {
    // rust supports a variety of math operations

    let addition = 5 + 4;
    let subtraction = 10 - 6;

    let multiply = 3 * 2;
    let division = 6 / 3;

    // when dividing an integer by another integer
    // rust performs floor division -> returns an int as the final value

    let various: i32 = (addition + subtraction) * (multiply) / division;
    println!("Computing: ({addition} + {subtraction}) * {multiply} / {division} = {various}");

    // if we want floating point value, we gotta divide using floating points

    let floats: f64 = 5.2 / 3.1;
    println!("Decimal division: {floats}");

    // modulo operator -> % returns the remainded of a division
    // basically what's left over after a division
    // ex:m 7 % 2, gives the remainder of 7 divided by 2

    let modulo = 7 % 2;
    println!("7 % 2 = {modulo}"); // goes into 6 evenly so we got 1 left over
}
