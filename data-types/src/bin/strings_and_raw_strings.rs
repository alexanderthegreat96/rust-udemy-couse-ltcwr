#[allow(unused_variables)]

fn print_as_chars(input: &str) {
    let splittable: Vec<char> = input.chars().collect::<Vec<char>>();

    for c in splittable {
        print!("|{c}| ");
    }
}

fn main() {
    println!("Dear Emily,\nHow have you been?\n Just testing this!");
    println!("\tOnce upon a time");
    println!("Juliet said \"I love you Romeo\""); // typical backslash usage here

    // let filepath = "C:\My Documents\new\videos";
    let filepath = r"C:\My Documents\new\videos"; // same thing as below, but because we added r in front of the string, it's a raw string
    // therefore, we no longer need to escape the backslashes
    println!("{filepath}");

    // escaping backslashes for example for windows

    let filepath2 = "C:\\My Documents\\new\\videos"; //  this is how you're supposed to do this
    println!("{filepath2}");

    print_as_chars(filepath2);
}
