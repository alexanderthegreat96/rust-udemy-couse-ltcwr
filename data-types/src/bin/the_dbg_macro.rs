fn main() {
    // the debug mecro
    // simply works similar to regular printing
    // but dirtier
    let seasons = ["Spring", "Summer", "Fall", "Winter"];

    println!("{}", 5);
    println!("{}", 3.14);
    println!("{}", true);
    println!("{seasons:#?}");

    dbg!(seasons); // the benefit of the developer and not the end user
}
