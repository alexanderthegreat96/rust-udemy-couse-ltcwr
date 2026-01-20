fn main() {
    // augmented assignment operators
    // when working with numeric data
    // it's common to use an existing numeric value
    // so we apply a methamatical operation to it

    let mut year: i32 = 2025;

    println!("Year: {year}");
    year += 1; // this is the augmented assignment operator
    println!("Year: {year}");

    // rust does not support ++ or --
    // doesn't work like c
    year -= 2;
    println!("Year: {year}");

    // we're reading them left to right essentially
    year *= 3;
    println!("Year: {year}");

    year /= 2;
    println!("Year: {year}");
}
