fn main() {
    // it's a compound type
    // hold multiple values within it basically
    // this is an array
    // it's a fixed size collection of homogenous data -> the data is of the same type
    // [type; length]
    let numbers: [i32; 6] = [4, 8, 15, 16, 23, 42];

    println!("My numbers: {:?}", numbers);

    let apples: [&str; 3] = ["Granny Smith", "McIntosh", "Red Delicious"];
    println!("Length: {}", apples.len());

    // when declaring an empty array
    // we need to provide the type
    // or simpy populate a value
    let mut currency_rates: [f64; 3] = [0.0; 3]; // should never do this, but this inits the array with 3 elements of 0
    // then we fill the elements
    // should use a growable array or a slice using a vector
    // !vec | Vec::new();
    currency_rates.fill(1.15);
    currency_rates.fill(2.22);
    currency_rates.fill(4.1);

    // i used fill to add more data
    println!("My currency rates: {:?}", currency_rates);
}
