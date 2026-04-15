use std::collections::HashSet;

fn main() {
    // set operations
    let mut concert_queue: HashSet<&str> = HashSet::new();
    let mut movie_queue: HashSet<&str> = HashSet::new();

    concert_queue.insert("Boris");
    concert_queue.insert("Melissa");

    movie_queue.insert("Boris");
    movie_queue.insert("Phil");

    // union gives the combinatation found in both sets
    // will return a union
    println!("{:?}", concert_queue.union(&movie_queue));
    println!("{:?}", movie_queue.union(&concert_queue));

    // gives values that are not found in both hashsets
    println!("{:?}", concert_queue.difference(&movie_queue));
    println!("{:?}", movie_queue.difference(&concert_queue));

    // gives values exclusive to the hashsets
    // but not common in both
    // basically unique in either sets, but not both
    println!("{:?}", concert_queue.symmetric_difference(&movie_queue));
    println!("{:?}", movie_queue.symmetric_difference(&concert_queue));

    // returns true if the sets have no values in common
    // useful to check if they have values in common
    println!("{}", concert_queue.is_disjoint(&movie_queue));
    println!("{}", movie_queue.is_disjoint(&concert_queue));

    let mut attendees = HashSet::new();
    attendees.insert("Boris");

    // the set is a subset of the argument set
    // are all of the entries in attendees in concerc queue?
    println!("{}", attendees.is_subset(&concert_queue));

    // are all the values in my attendes found within concert queue
    // it makes attendees a subset of concert queue
    println!("{}", concert_queue.is_superset(&attendees));
}
