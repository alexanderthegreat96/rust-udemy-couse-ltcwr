use std::collections::HashSet;

fn main() {
    // a hashset is called a set in other languages
    // it basically ensures we only store unique values
    // example: set from python does the thing

    let mut concert_queue: HashSet<&str> = HashSet::new();
    println!("{:?}", concert_queue);

    concert_queue.insert("Molly");
    concert_queue.insert("Megan");
    println!("{:?}", concert_queue);
    println!("{}", concert_queue.len());

    concert_queue.insert("Molly");
    println!("{:?}", concert_queue);

    println!("{}", concert_queue.remove("Megan"));
    println!("{}", concert_queue.remove("Franny"));
    println!("{:?}", concert_queue);

    println!("{}", concert_queue.contains("Molly"));
    println!("{}", concert_queue.contains("Fred"));

    println!("{:?}", concert_queue.get("Molly"));
    println!("{:?}", concert_queue.get("Joe"));

    // As you can see, the hashset rejects entries
    // that are non unique
    let mut users_queue: HashSet<&str> = HashSet::new();

    users_queue.insert("Mike");
    users_queue.insert("James");
    users_queue.insert("Dan");
    users_queue.insert("Dan");
    users_queue.insert("James");

    if let exists = users_queue.contains("Dan") {
        println!("We just found Dan in the queue");
    }

    println!("Users queue: {:?}", users_queue);
    // get method returns a reference to the data inside the hashset
    // so ownership does not get moved
    println!("Getting Mike: {:?}", users_queue.get("Mike"));
}
