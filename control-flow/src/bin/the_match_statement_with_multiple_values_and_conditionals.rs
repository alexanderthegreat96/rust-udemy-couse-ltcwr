fn main() {
    let number = 8;
    let names: [&str; 5] = ["mike", "josh", "dan", "joseph", "john"];

    // we can match on conditionals too
    match number {
        value if value % 2 == 0 => println!("{value} is an even number"),
        value if value % 2 != 0 => println!("{value} is an odd number"),
        _ => unreachable!(),
    }

    // doing a match on the names
    for name in names {
        // match name {
        //     value if value == "mike" => {
        //         println!("Found mike!")
        //     }
        //     value if value == "josh" => {
        //         println!("Found josh!")
        //     }
        //     _ => {}
        // }

        // this is better than the one above
        // because we can match if multiple values
        // are found
        // of course, same condition here
        match name {
            "mike" | "josh" | "joseph" => {
                println!("We found: {}", name);
            }
            _ => {}
        }
    }
}
