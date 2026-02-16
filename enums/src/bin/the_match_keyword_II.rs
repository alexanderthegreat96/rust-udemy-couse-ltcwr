/// The Debug attribute allows the enum to be printed for testing purposes.
#[derive(Debug)]
enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
}

fn main() {
    // We initialize 'my_computer'.
    // Note: Passing this to the function will MOVE it because it doesn't implement Copy.
    let my_computer = OperatingSystem::MacOS;
    let age = years_since_release(my_computer);
    println!("My computer's operating system is {age} years old");

    let dads_computer = OperatingSystem::Windows;
    let age = years_since_release(dads_computer);
    println!("My dad's computer is {age} years old");
}

/// A function that maps an Enum variant to a u32 value.
fn years_since_release(os: OperatingSystem) -> u32 {
    // A match expression evaluates to the value of the chosen arm.
    match os {
        OperatingSystem::Windows => {
            // Match arms can contain multiple statements.
            println!("Quite an old operating system!");
            39 // The last line without a semicolon is the return value for this arm.
        }
        OperatingSystem::MacOS => {
            // You can execute side effects (like function calls) before returning a value.
            call_this();
            20
        }
        OperatingSystem::Linux => {
            // For simple returns, you don't even need the curly braces {}.
            25
        }
    }
}

/// A simple function that returns 'Unit' (equivalent to None/Void).
fn call_this() -> () {
    println!("This was called!");
}
