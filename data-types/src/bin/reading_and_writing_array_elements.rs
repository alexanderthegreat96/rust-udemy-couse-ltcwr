use std::str;

// we added lifetine operator here
// because wwant to ensure that name does not outlive names
// so this way they do not
// also, the compiler doesn't rape us
// the reason why we add <'a, const N: usize>
// is because we declare the values before arguments
// the array has a constant size of type usizee and a lifetime arg
fn change_name<'a, const N: usize>(index: usize, name: &'a str, names: &mut [&'a str; N]) {
    if index > names.len() {
        return;
    }

    names[index] = name;
}

fn main() {
    // mutating arrays
    // we're doing it using indexes
    // since they are 0 indexed
    // aka 0 to n
    let mut seasons = ["Spring", "Summer", "Fall", "Winter"];

    println!("{}", seasons[2]);
    seasons[2] = "Autumn";
    println!("{}", seasons[2]);

    let mut names: [&str; 3] = ["Michael", "Jordan", "Smith"];

    println!("First item before: {}", names[0]);
    change_name(0, "Demetriot Titus", &mut names);
    println!("First item after: {}", names[0]);
}
