fn main() {
    // works the same way as it would in a normal array
    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let mut pizza_toppings = vec![pepperoni, mushroom, sausage];

    pizza_toppings[1] = String::from("Olives");
    println!("{pizza_toppings:#?}");

    // this is where we're targetting a certain string into that element
    // so we basically take a mutable reference to that element
    // then we simply modify it
    // the reason why we use a mutable reference is because we don't
    // want to transfer ownership

    let target_topping = &mut pizza_toppings[2];
    target_topping.push_str(" and Meatballs");
    // then we reassign the modified string at position index 2
    // quite logical
    let another_topping = &pizza_toppings[2];
    let another_one = &pizza_toppings[2];
    println!("{another_topping} {another_one}");
    println!("{pizza_toppings:#?}");


    let mut container : StringContainer = StringContainer::new();
    container.add_string("Car");
    container.add_string("Jag");
    container.add_string("Mouse");
    container.add_string("Cat");

    container.modify_string(2, " and Cat"); 
    container.print_items();
    container.add_string("Something");
    container.remove_last();
    container.print_items();
    container.remove_string(1);
    container.print_items();

}

// just a rather interesting string container
struct StringContainer {
    strings: Vec<String>
}

impl StringContainer {
    fn new() -> Self {
        StringContainer { strings: Vec::<String>::new() }
    }

    fn add_string(&mut self, string: &str) {
        let to_str = string.to_string();
        self.strings.push(to_str);
    }

    fn remove_string(&mut self, idx: usize) -> bool {
        if let Some(_) = self.strings.get(idx) {
            self.strings.remove(idx);
            return true;

        } else {
            return false;
        }
    }


    fn remove_last(&mut self) {
        self.strings.pop();
    }

    fn modify_string(&mut self, idx: usize, to_add: &str) -> bool {
        if let Some(current_string) = self.strings.get(idx) {
            let mut current_str = current_string.to_string();
            current_str.push_str(to_add);

            self.strings.insert(idx, current_str);
            true
        } else {
            false
        }
    }

    fn print_items(&self) {
        if self.strings.is_empty() {
            return;
        }

        for item in &self.strings {
            println!("{:?}", item);
        }
    }
} 
