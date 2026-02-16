/// The Debug trait allows the enum to be printed or inspected during debugging.
#[derive(Debug)]
enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
}

fn main() {
    // We initialize the variable with a specific variant.
    let my_computer = OperatingSystem::MacOS;

    // The function call moves 'my_computer' into 'years_since_release'.
    let age = years_since_release(my_computer);

    println!("My computer's operating system is {age} years old");
}

/// This function demonstrates the 'Match as an Expression' pattern.
/// The compiler ensures every variant of OperatingSystem is handled.
fn years_since_release(os: OperatingSystem) -> u32 {
    // Because the match is the last (and only) expression in the function,
    // its resulting value is implicitly returned.

    match os {
        OperatingSystem::Windows => 39, // No semicolon = return value
        OperatingSystem::MacOS => 20,
        OperatingSystem::Linux => 25,
        // If you were to add 'Android' to the enum, this code would
        // immediately fail to compile until a case for Android is added.
    }
}
