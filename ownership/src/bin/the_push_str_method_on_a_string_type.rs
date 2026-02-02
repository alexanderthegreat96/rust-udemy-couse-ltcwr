fn main() {
    // the content lives on the heap
    // while the string info stays on the heap;
    // refference
    // length
    // capacity

    let mut name = String::from("Boris");
    assert_eq!(name, "Boris".to_string());

    println!("{name}");

    // this concatenates strings
    name.push_str(" Pask");
    println!("{name}");

    name.push_str("haver");
    println!("{name}");
}
