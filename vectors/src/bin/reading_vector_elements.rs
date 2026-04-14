fn main() {
    // to read values from the vectors
    // we can use indexes directly
    // or slice operations
    let pizza_diameters = vec![8, 10, 12, 14];

    println!("Chosen diameter: {}", pizza_diameters.get(2).unwrap());

    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let pizza_toppings = vec![pepperoni, mushroom, sausage];

    // read everything from the second element to the end
    let pizza_slice = &pizza_toppings[1..];
    println!("{pizza_slice:?}");

    // Just slice operations here
    let mut cars: Vec<String> = Vec::from(
    [
        "BMW x5".to_string(),
        "Rolls Royce Phantom".to_string(),
        "Ford Mustang GT".to_string(),
        "Alfa Romeo Giulia".to_string(),
        "Audi A8".to_string(),
        "Ford GT".to_string(),
        "Volkswagen Touareg".to_string(),
        "Audi Q7 V12 TDI".to_string()
    ]);

    println!("All cars after third car: {:?}", &cars[3..]);
    println!("Cars between Ford Mustang GT and Volkswagen Touareg: {:?}", &cars[3..6]);
    
    // using the vec length and substracting 2
    println!("The last 2 cars: {:?}", &cars[cars.len()-2..]);

    // You can also reverse the whole thing
    cars.reverse();
    println!("The last 2 cars: {:?}", &cars[..2]);

}
