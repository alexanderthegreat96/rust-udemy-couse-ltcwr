fn main() {
    let pi: f64 = 3.1415926535897932384;

    // we can use the : to create a format specifier
    // the format specifier enables customization of the digital
    // represensation within this printed string
    // {:2.} -> give me only 2 digits after the floating point
    println!("The current value of pi is {:.4}", pi);
    println!("The current value of pi is {:.2}", pi);
}
