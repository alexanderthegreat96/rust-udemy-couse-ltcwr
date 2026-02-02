fn main() {
    /*
      String - A dynamic piece of text stored on the heap
      at runtime

      &String ("ref String") - A reference to a heap String

      str - A hardcoded, read-only piece of text encoded in
      the binary

      &str ("ref str") - A reference to th text in the memory
      that has loaded the binary file
    */

    let ice_cream: &str = "Cookies and Cream"; // string literal -> this is embedded into the binary exetuable. the value is known at compile time
    let pizza: String = String::from("Pineapple Pizza"); // a growable strings -> stored in the heap

    println!("Icecream: {}, Pizza: {}", ice_cream, pizza);

    let mut text: String = String::new();
    text.push_str("I pushed a string into this string.");
    println!("Created growable string: {}", text);
}
