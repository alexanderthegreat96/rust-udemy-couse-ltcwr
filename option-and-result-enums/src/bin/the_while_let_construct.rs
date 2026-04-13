fn main() {
    // this is a very interesting one
    let mut sauces = vec!["Mayonaise", "Ketchup", "Ranch"];
    
    // basically same as If let
    // but it will work in a loop
    while let Some(sauce) = sauces.pop() {
        println!("The next sauce is {sauce}");
    }
}


