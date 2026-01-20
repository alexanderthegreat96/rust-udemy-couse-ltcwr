fn main() {
    // floating point types
    // f32 and f64
    // most of the type the f64 should be enough most of the time
    // f32 is generally used when we care a little too much about performance
    // rusts default float type is f64

    let x: f32 = 3.14; // 32 bit floating point
    let y: f64 = 2.718281828459045; // 64 bit floating point
    println!("x: {}, y: {}", x, y);

    let sum = x + y as f32; // type casting y to f32 -> cool ha?
    println!("Sum: {}", sum);

    println!("{}", y.floor()); // rounds down
    println!("{}", y.ceil()); // rounds up
}
