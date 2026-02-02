fn main() {
    // the copy trait
    // when a type implements a trait
    // then the type promises what the trait mandates

    // this example triggers an automatic
    // copy
    // in this case, i32 implements the copy trait
    let time = 2025;
    // years copies time
    let years = time;

    println!("The time is {time}. It is the year {years}.");
}
