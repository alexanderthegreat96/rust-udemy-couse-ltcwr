fn main() {
    // an array slice
    // with a string
    // the numbers in the brackets represent bytes
    // with an array
    // they represent array indexes
    let values = [4, 8, 15, 16, 23, 42];

    // starting up until the fouth
    let my_slice = &values[..4];
    println!("{my_slice:?}");

    let my_slice = &values[2..4];
    println!("{my_slice:?}");

    // third element to the end
    let my_slice = &values[2..];
    println!("{my_slice:?}");

    // start to end
    let my_slice = &values[..];
    println!("{my_slice:?}");

    // the entire clice
    let my_slice = &values;
    println!("{my_slice:?}");
}
