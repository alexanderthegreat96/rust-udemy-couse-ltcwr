fn main() {
    // every vector has a capacity
    // rust will always find a different place
    // in memory with increased capacity
    // sometimes it might move elements in a different position
    // we can, however, specify a max limit
    // with_capacity
    // once the capacity is surpassed
    // it will panic
    let mut seasons: Vec<&str> = Vec::with_capacity(4);
    println!(
        "Length: {}. Capacity: {}",
        seasons.len(),
        seasons.capacity(),
    );

    seasons.push("Summer");
    seasons.push("Fall");
    seasons.push("Winter");
    seasons.push("Spring");
    println!(
        "Length: {}. Capacity: {}",
        seasons.len(),
        seasons.capacity(),
    );

    seasons.push("Summer");

    println!(
        "Length: {}. Capacity: {}",
        seasons.len(),
        seasons.capacity(),
    );
}
