fn main() {
    let action_hero = String::from("Arnold Schwarzenegger");

    // we're borrowing a portion of the string
    // a slice is a refference to a collection of elements
    // we're grabbing a range of bytes to borrow
    let first_name: &str = &action_hero[0..6];
    println!("{first_name}");

    // as a general idea
    // we need to ensure that our range fits inside the buffer
    // // in other words, we are grabbing chearacters 7..21
    // if we were to use 22, it would be out of bounds
    // so it's generally important to actually check the bounds
    let last_name = &action_hero[7..20];
    println!("{last_name}");

    let new_str: String = String::from("The Avangers");

    let extracted: &str = extract_str(&new_str, 3, 7).unwrap_or("not-found");
    println!(
        "Using: {} to extact the 3rd to 7th characters: {:?}",
        new_str, extracted
    );
}

// extacting the string slice in here
// ownership is not moved here
// the &str refference does not "steal ownership"
fn extract_str(full: &str, start: usize, end: usize) -> Option<&str> {
    if start > end || end > full.len() {
        return None;
    }

    Some(&full[start..end])
}
