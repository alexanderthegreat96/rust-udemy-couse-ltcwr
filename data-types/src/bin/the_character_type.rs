fn split_chars_and_print(string: String) {
    let char_list: Vec<char> = string.chars().collect();
    let mut char_dummy: Vec<char> = Vec::new();

    let mut i: usize = 0;
    for char in &char_list {
        char_dummy.push(*char);
        if i < char_list.len() - 1 {
            char_dummy.push('|');
        }

        i += 1;
    }

    let result: String = char_dummy.into_iter().collect();
    println!("{}", result);
}

fn main() {
    // the character type
    // the character type is a sub division of the string type
    // virtually, all strings can be split into characters
    // unicode is a computing standard for the representation
    // of text in compute systems
    // it support emojis, symbols and more
    // UTF = unicode transformation format

    let first_initial = 'B';
    let emoji = '🎧';

    println!(
        "{} {}",
        first_initial.is_alphabetic(),
        emoji.is_alphabetic()
    );

    println!("{} {}", first_initial.is_uppercase(), emoji.is_uppercase());
    println!("{} {}", first_initial.is_lowercase(), emoji.is_lowercase());

    split_chars_and_print(String::from("Michael"));
}
