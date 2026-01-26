fn even_or_odd(number: i32) {
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("The number is {result}");
}

fn is_alex(name: &str) -> bool {
    // this entire construct can be assigned to a variable
    // it's a basic inline statement so we would close it with ;
    let result: bool = if name == "alex" { true } else { false };
    return result;
}

fn main() {
    even_or_odd(17);
    even_or_odd(100);

    println!("Is this Alex?: {}", is_alex("damian"));
}
