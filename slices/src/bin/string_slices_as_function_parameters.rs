// this is a str slice reference
fn do_hero_stuff(hero_name: &str) {
    // &String -> &str
    println!("{hero_name} saves the day!");
}

// this is a String ref
fn something_else(input: &String) {
    println!("Doing something else: {}", input);
}

fn main() {
    let action_hero = String::from("Arnold Schwarzenegger");
    do_hero_stuff(&action_hero);

    // we could have sliced the action hero and the result is the same
    // since that would also be a str ref -> &str
    let another_action_hero = "Sylvester Stallone";
    do_hero_stuff(another_action_hero);
    something_else(&action_hero);
}
