#[derive(Debug)]
struct DeliSandwich {}

fn main() {
    println!("{}", identity::<i32>(5));
    println!("{}", identity::<i8>(5));
    println!("{}", identity::<u32>(5));
    println!("{}", identity::<f64>(13.14));
    println!("{}", identity::<&str>("hello"));
    println!("{}", identity::<String>(String::from("hello")));
    println!("{}", identity::<bool>(true));
    println!("{:?}", identity::<DeliSandwich>(DeliSandwich {}));

    print_type_of::<String>(String::from("Michael"));
    print_type_of::<i32>(34);
    print_type_of(12.3);
    print_type_of("someone");
}

// turbofish -> ::<i32>
// it's when you specify the actual type
// whatever is in the brackets is the type you're specifying
fn identity<T>(value: T) -> T {
    value
}

// here T implements std::fmt::Display
fn print_type_of<T: std::fmt::Display>(val: T) {
    println!(
        "Value: {} is of type: {}",
        val,
        std::any::type_name_of_val(&val)
    );
}
